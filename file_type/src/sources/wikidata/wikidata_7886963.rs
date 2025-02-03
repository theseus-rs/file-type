use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7886963: FileFormat = FileFormat {
    id: 7_886_963,
    source_type: SourceType::Wikidata,
    name: "Uniqueness Database File",
    extensions: &["udf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
