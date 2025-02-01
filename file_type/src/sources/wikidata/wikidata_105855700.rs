use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855700: FileFormat = FileFormat {
    id: 105_855_700,
    puid: "wikidata/105855700",
    name: "OpenVPN profile (with rem)",
    extensions: &["ovpn"],
    media_types: &["application/x-openvpn-profile"],
    internal_signatures: &[],
    related_formats: &[],
};
