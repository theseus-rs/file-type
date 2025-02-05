use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4875438: FileFormat = FileFormat {
    id: 4_875_438,
    source_type: SourceType::Wikidata,
    name: "Be-Music Source",
    extensions: &["bms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
