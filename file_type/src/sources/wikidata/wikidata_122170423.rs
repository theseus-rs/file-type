use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122170423: FileFormat = FileFormat {
    id: 122_170_423,
    source_type: SourceType::Wikidata,
    name: "AmnezaVPN profile",
    extensions: &["vpn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
