use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7886963: FileFormat = FileFormat {
    id: 7_886_963,
    source_type: SourceType::Wikidata,
    name: "Uniqueness Database File",
    extensions: &["udf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
