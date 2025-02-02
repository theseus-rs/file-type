use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855700: FileFormat = FileFormat {
    id: 105_855_700,
    source_type: SourceType::Wikidata,
    name: "OpenVPN profile (with rem)",
    extensions: &["ovpn"],
    media_types: &["application/x-openvpn-profile"],
    internal_signatures: &[],
    related_formats: &[],
};
