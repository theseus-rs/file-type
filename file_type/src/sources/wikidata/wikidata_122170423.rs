use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122170423: FileFormat = FileFormat {
    id: 122_170_423,
    source_type: SourceType::Wikidata,
    name: "AmnezaVPN profile",
    extensions: &["vpn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
