use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59914669: FileFormat = FileFormat {
    id: 59_914_669,
    source_type: SourceType::Wikidata,
    name: "Steel Detailing Neutral Format",
    extensions: &["sdn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
