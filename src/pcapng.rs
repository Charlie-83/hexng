use crate::util::div_ceil;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Stylize},
  text::{Line, Span},
  widgets::{Paragraph, Widget, Wrap},
};

#[derive(Eq, PartialEq)]
pub enum BlockErrorKind {
  None,
  ZeroLength,
}

pub struct PngBlock {
  pub id: u32,
  pub raw: Vec<u8>,
  pub block_type: u32,
  pub length: u32,
  pub options: Vec<u8>,
  pub error: BlockErrorKind,
}

pub fn parse(data: &Vec<u8>) -> Vec<PngBlock> {
  let mut out: Vec<PngBlock> = vec![];
  let mut pos: usize = 0;
  let mut id: u32 = 0;
  while pos < data.len() {
    let block_type = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
    let length = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap());
    if length == 0 {
      out.push(PngBlock::new(
        vec![],
        block_type,
        length,
        vec![],
        id,
        BlockErrorKind::ZeroLength,
      ));
      break;
    }
    out.push(PngBlock::new(
      data[pos..pos + (length as usize)].to_vec(),
      block_type,
      length,
      vec![],
      id,
      BlockErrorKind::None,
    ));
    id += 1;
    pos += length as usize;
  }
  out
}

impl PngBlock {
  fn new(
    raw: Vec<u8>,
    block_type: u32,
    length: u32,
    options: Vec<u8>,
    id: u32,
    error: BlockErrorKind,
  ) -> PngBlock {
    PngBlock {
      raw,
      block_type,
      length,
      options,
      id,
      error,
    }
  }

  pub fn draw(
    &self,
    mut area: Rect,
    buf: &mut Buffer,
    hidden: u16,
    folded: bool,
    ascii: bool,
  ) -> u16 {
    if self.error == BlockErrorKind::ZeroLength {
      Line::raw(self.id.to_string() + ": ERROR Block has zero length")
        .underlined()
        .bold()
        .render(area, buf);
    }

    let bytes_in_row = (area.width + 1) / 3;
    let total_rows = div_ceil(self.length as u16, bytes_in_row) + 1;
    assert!(hidden < total_rows);

    let total_rows_to_print = std::cmp::min(total_rows - hidden, area.height);
    if total_rows_to_print == 0 {
      return 0;
    }
    let mut rows_to_print = total_rows_to_print;

    if self.block_type == 1 {
      Line::raw(
        self.id.to_string()
          + ": "
          + &block_type_str(self.block_type)
          + " - "
          + &link_type_str(self.raw[8] as u32 + ((self.raw[9] as u32) << 8)),
      )
      .underlined()
      .bold()
      .render(area, buf);
    } else {
      Line::raw(self.id.to_string() + ": " + &block_type_str(self.block_type))
        .underlined()
        .bold()
        .render(area, buf);
    }
    if folded || rows_to_print == 1 {
      return 1;
    }

    area.y += 1;
    area.height -= 1;
    rows_to_print -= 1;

    let print_bytes: fn(&[u8]) -> String = if ascii { to_ascii } else { to_hex };

    let start: usize = (hidden * bytes_in_row) as usize;
    let end: usize =
      std::cmp::min((hidden + rows_to_print) * bytes_in_row, self.length as u16) as usize;
    let mut spans = vec![];
    let mut current_section = 0;
    let mut index = 0;
    let mut fg_colour_index = 0;
    let mut bg_colour_index = 0;
    let fg_colours = [
      Color::White,
      Color::Red,
      Color::Green,
      Color::Magenta,
      Color::LightBlue,
    ];
    let bg_colours = [Color::Black, Color::DarkGray];
    while index < end {
      let section = self.sections()[current_section];
      if index < start && index + section > start {
        spans.push(
          Span::raw(print_bytes(&self.raw[start..index + section]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      } else if index >= start && index + section <= end {
        spans.push(
          Span::raw(print_bytes(&self.raw[index..index + section]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      } else if index < end && index + section > end {
        spans.push(
          Span::raw(print_bytes(&self.raw[index..end]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      }
      fg_colour_index = (fg_colour_index + 1) % fg_colours.len();
      bg_colour_index = (bg_colour_index + 1) % bg_colours.len();
      index += section;
      current_section += 1;
    }

    for i in 0..spans.len() {
      spans.insert(2 * i + 1, Span::raw("|"));
    }

    Paragraph::new(Line::from(spans))
      .wrap(Wrap { trim: true })
      .render(area, buf);
    total_rows_to_print
  }

  pub fn rows(&self, width: u16) -> u16 {
    if self.error == BlockErrorKind::ZeroLength {
      return 1;
    }
    let bytes_in_row = (width + 1) / 3;
    div_ceil(self.length as u16, bytes_in_row) + 1
  }

  fn sections(&self) -> Vec<usize> {
    let sections = match self.block_type {
      0x00000006 => vec![4, 4, 4, 4, 4, 4, 4, self.length as usize - 32, 4],
      0x0A0D0D0A => vec![4, 4, 4, 2, 2, 8, self.length as usize - 28, 4],
      0x00000001 => vec![4, 4, 2, 2, 4, self.length as usize - 20, 4],
      _ => vec![4, 4, self.length as usize - 12, 4],
    };
    assert_eq!(sections.iter().map(|s| *s as u32).sum::<u32>(), self.length);
    sections
  }
}

fn to_hex(s: &[u8]) -> String {
  s.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<_>>()
    .join(" ")
}

fn to_ascii(s: &[u8]) -> String {
  s.iter()
    .map(|&b| {
      if b > 32 && b < 127 {
        char::from(b)
      } else {
        '.'
      }
    })
    .map(|c| c.to_string() + " ")
    .collect::<Vec<_>>()
    .join(" ")
}

fn block_type_str(block_type: u32) -> String {
  match block_type {
    0x00000001 => "Interface Description Block".to_owned(),
    0x00000002 => "Packet Block".to_owned(),
    0x00000003 => "Simple Packet Block".to_owned(),
    0x00000004 => "Name Resolution Block".to_owned(),
    0x00000005 => "Interface Statistics Block".to_owned(),
    0x00000006 => "Enhanced Packet Block".to_owned(),
    0x00000007 => "IRIG Timestamp/Socket Aggregation Event Block".to_owned(),
    0x00000008 => "AFDX Encapsulation Information Block".to_owned(),
    0x00000009 => "systemd Journal Export Block".to_owned(),
    0x0000000a => "Decryption Secrets Block".to_owned(),
    0x00000101 => "Hone Project Machine Info Block".to_owned(),
    0x00000102 => "Hone Project Connection Event Block".to_owned(),
    0x00000201 => "Sysdig Machine Info Block".to_owned(),
    0x00000202 => "Sysdig Process Info Block, version 1".to_owned(),
    0x00000203 => "Sysdig FD List Block".to_owned(),
    0x00000204 => "Sysdig Event Block".to_owned(),
    0x00000205 => "Sysdig Interface List Block".to_owned(),
    0x00000206 => "Sysdig User List Block".to_owned(),
    0x00000207 => "Sysdig Process Info Block, version 2".to_owned(),
    0x00000208 => "Sysdig Event Block with flags".to_owned(),
    0x00000209 => "Sysdig Process Info Block, version 3".to_owned(),
    0x00000210 => "Sysdig Process Info Block, version 4".to_owned(),
    0x00000211 => "Sysdig Process Info Block, version 5".to_owned(),
    0x00000212 => "Sysdig Process Info Block, version 6".to_owned(),
    0x00000213 => "Sysdig Process Info Block, version 7".to_owned(),
    0x00000BAD => "Custom Block that rewriters can copy into new files".to_owned(),
    0x40000BAD => "Custom Block that rewriters should not copy into new files".to_owned(),
    0x0A0D0D0A => "Section Header Block".to_owned(),
    _ => "Unknown".to_owned(),
  }
}

fn link_type_str(link_type: u32) -> String {
  match link_type {
    0 => "Null".to_owned(),
    1 => "Ethernet".to_owned(),
    2 => "Exp_ethernet".to_owned(),
    3 => "Ax25".to_owned(),
    4 => "Pronet".to_owned(),
    5 => "Chaos".to_owned(),
    6 => "Ieee802_5".to_owned(),
    7 => "Arcnet_bsd".to_owned(),
    8 => "Slip".to_owned(),
    9 => "Ppp".to_owned(),
    10 => "Fddi".to_owned(),
    50 => "Ppp_hdlc".to_owned(),
    51 => "Ppp_ether".to_owned(),
    99 => "Symantec_firewall".to_owned(),
    100 => "Atm_rfc1483".to_owned(),
    101 => "Raw".to_owned(),
    104 => "C_hdlc".to_owned(),
    105 => "Ieee802_11".to_owned(),
    106 => "Atm_clip".to_owned(),
    107 => "Frelay".to_owned(),
    108 => "Loop".to_owned(),
    109 => "Enc".to_owned(),
    112 => "Netbsd_hdlc".to_owned(),
    113 => "Linux_sll".to_owned(),
    114 => "Ltalk".to_owned(),
    117 => "Pflog".to_owned(),
    119 => "Ieee802_11_prism".to_owned(),
    122 => "Ip_over_fc".to_owned(),
    123 => "Sunatm".to_owned(),
    127 => "Ieee802_11_radiotap".to_owned(),
    128 => "Tzsp".to_owned(),
    129 => "Arcnet_linux".to_owned(),
    130 => "Juniper_mlppp".to_owned(),
    131 => "Juniper_mlfr".to_owned(),
    132 => "Juniper_es".to_owned(),
    133 => "Juniper_ggsn".to_owned(),
    134 => "Juniper_mfr".to_owned(),
    135 => "Juniper_atm2".to_owned(),
    136 => "Juniper_services".to_owned(),
    137 => "Juniper_atm1".to_owned(),
    138 => "Apple_ip_over_ieee1394".to_owned(),
    139 => "Mtp2_with_phdr".to_owned(),
    140 => "Mtp2".to_owned(),
    141 => "Mtp3".to_owned(),
    142 => "Sccp".to_owned(),
    143 => "Docsis".to_owned(),
    144 => "Linux_irda".to_owned(),
    145 => "Ibm_sp".to_owned(),
    146 => "Ibm_sn".to_owned(),
    147..=162 => "Private".to_owned(),
    163 => "Ieee802_11_avs".to_owned(),
    164 => "Juniper_monitor".to_owned(),
    165 => "Bacnet_ms_tp".to_owned(),
    166 => "Ppp_pppd".to_owned(),
    167 => "Juniper_pppoe".to_owned(),
    168 => "Juniper_pppoe_atm".to_owned(),
    169 => "Gprs_llc".to_owned(),
    170 => "Gpf_t".to_owned(),
    171 => "Gpf_f".to_owned(),
    172 => "Gcom_t1e1".to_owned(),
    173 => "Gcom_serial".to_owned(),
    174 => "Juniper_pic_peer".to_owned(),
    175 => "Erf_eth".to_owned(),
    176 => "Erf_pos".to_owned(),
    177 => "Linux_lapd".to_owned(),
    178 => "Juniper_ether".to_owned(),
    179 => "Juniper_ppp".to_owned(),
    180 => "Juniper_frelay".to_owned(),
    181 => "Juniper_chdlc".to_owned(),
    182 => "Mfr".to_owned(),
    183 => "Juniper_vp".to_owned(),
    184 => "A429".to_owned(),
    185 => "A653_icm".to_owned(),
    186 => "Usb_freebsd".to_owned(),
    187 => "Bluetooth_hci_h4".to_owned(),
    188 => "Ieee802_16_mac_cps".to_owned(),
    189 => "Usb_linux".to_owned(),
    190 => "Can20b".to_owned(),
    191 => "Ieee802_15_4_linux".to_owned(),
    192 => "Ppi".to_owned(),
    193 => "Ieee802_16_mac_cps_radio".to_owned(),
    194 => "Juniper_ism".to_owned(),
    195 => "Ieee802_15_4_withfcs".to_owned(),
    196 => "Sita".to_owned(),
    197 => "Erf".to_owned(),
    198 => "Raif1".to_owned(),
    199 => "Ipmb_kontron".to_owned(),
    200 => "Juniper_st".to_owned(),
    201 => "Bluetooth_hci_h4_with_phdr".to_owned(),
    202 => "Ax25_kiss".to_owned(),
    203 => "Lapd".to_owned(),
    204 => "Ppp_with_dir".to_owned(),
    205 => "C_hdlc_with_dir".to_owned(),
    206 => "Frelay_with_dir".to_owned(),
    207 => "Lapb_with_dir".to_owned(),
    209 => "I2c_linux".to_owned(),
    210 => "Flexray".to_owned(),
    211 => "Most".to_owned(),
    212 => "Lin".to_owned(),
    213 => "X2e_serial".to_owned(),
    214 => "X2e_xoraya".to_owned(),
    215 => "Ieee802_15_4_nonask_phy".to_owned(),
    216 => "Linux_evdev".to_owned(),
    217 => "Gsmtap_um".to_owned(),
    218 => "Gsmtap_abis".to_owned(),
    219 => "Mpls".to_owned(),
    220 => "Usb_linux_mmapped".to_owned(),
    221 => "Dect".to_owned(),
    222 => "Aos".to_owned(),
    223 => "Wihart".to_owned(),
    224 => "Fc_2".to_owned(),
    225 => "Fc_2_with_frame_delims".to_owned(),
    226 => "Ipnet".to_owned(),
    227 => "Can_socketcan".to_owned(),
    228 => "Ipv4".to_owned(),
    229 => "Ipv6".to_owned(),
    230 => "Ieee802_15_4_nofcs".to_owned(),
    231 => "Dbus".to_owned(),
    232 => "Juniper_vs".to_owned(),
    233 => "Juniper_srx_e2e".to_owned(),
    234 => "Juniper_fibrechannel".to_owned(),
    235 => "Dvb_ci".to_owned(),
    236 => "Mux27010".to_owned(),
    237 => "Stanag_5066_d_pdu".to_owned(),
    238 => "Juniper_atm_cemic".to_owned(),
    239 => "Nflog".to_owned(),
    240 => "Netanalyzer".to_owned(),
    241 => "Netanalyzer_transparent".to_owned(),
    242 => "Ipoib".to_owned(),
    243 => "Mpeg_2_ts".to_owned(),
    244 => "Ng40".to_owned(),
    245 => "Nfc_llcp".to_owned(),
    246 => "Pfsync".to_owned(),
    247 => "Infiniband".to_owned(),
    248 => "Sctp".to_owned(),
    249 => "Usbpcap".to_owned(),
    250 => "Rtac_serial".to_owned(),
    251 => "Bluetooth_le_ll".to_owned(),
    252 => "Wireshark_upper_pdu".to_owned(),
    253 => "Netlink".to_owned(),
    254 => "Bluetooth_linux_monitor".to_owned(),
    255 => "Bluetooth_bredr_bb".to_owned(),
    256 => "Bluetooth_le_ll_with_phdr".to_owned(),
    257 => "Profibus_dl".to_owned(),
    258 => "Pktap".to_owned(),
    259 => "Epon".to_owned(),
    260 => "Ipmi_hpm_2".to_owned(),
    261 => "Zwave_r1_r2".to_owned(),
    262 => "Zwave_r3".to_owned(),
    263 => "Wattstopper_dlm".to_owned(),
    264 => "Iso_14443".to_owned(),
    265 => "Rds".to_owned(),
    266 => "Usb_darwin".to_owned(),
    267 => "Openflow".to_owned(),
    268 => "Sdlc".to_owned(),
    269 => "Ti_lln_sniffer".to_owned(),
    270 => "Loratap".to_owned(),
    271 => "Vsock".to_owned(),
    272 => "Nordic_ble".to_owned(),
    273 => "Docsis31_xra31".to_owned(),
    274 => "Ethernet_mpacket".to_owned(),
    275 => "Displayport_aux".to_owned(),
    276 => "Linux_sll2".to_owned(),
    277 => "Sercos_monitor".to_owned(),
    278 => "Openvizsla".to_owned(),
    279 => "Ebhscr".to_owned(),
    280 => "Vpp_dispatch".to_owned(),
    281 => "Dsa_tag_brcm".to_owned(),
    282 => "Dsa_tag_brcm_prepend".to_owned(),
    283 => "Ieee802_15_4_tap".to_owned(),
    284 => "Dsa_tag_dsa".to_owned(),
    285 => "Dsa_tag_edsa".to_owned(),
    286 => "Elee".to_owned(),
    287 => "Z_wave_serial".to_owned(),
    288 => "Usb_2_0".to_owned(),
    289 => "Atsc_alp".to_owned(),
    290 => "Etw".to_owned(),
    291 => "Netanalyzer_ng".to_owned(),
    292 => "Zboss_ncp".to_owned(),
    293 => "Usb_2_0_low_speed".to_owned(),
    294 => "Usb_2_0_full_speed".to_owned(),
    295 => "Usb_2_0_high_speed".to_owned(),
    296 => "Auerswald_log".to_owned(),
    297 => "Zwave_tap".to_owned(),
    298 => "Silabs_debug_channel".to_owned(),
    299 => "Fira_uci".to_owned(),
    300 => "Mdb".to_owned(),
    301 => "Dect_nr".to_owned(),
    _ => "Unknown".to_owned(),
  }
}
