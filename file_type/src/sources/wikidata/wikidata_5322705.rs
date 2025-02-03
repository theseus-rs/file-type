use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5322705: FileFormat = FileFormat {
    id: 5_322_705,
    source_type: SourceType::Wikidata,
    name: "Extended Common Object File Format",
    extensions: &["o", "so"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
