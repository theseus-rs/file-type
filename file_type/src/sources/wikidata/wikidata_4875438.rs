use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4875438: FileFormat = FileFormat {
    id: 4_875_438,
    source_type: SourceType::Wikidata,
    name: "Be-Music Source",
    extensions: &["bms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
