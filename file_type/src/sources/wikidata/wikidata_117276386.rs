use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117276386: FileFormat = FileFormat {
    id: 117_276_386,
    source_type: SourceType::Wikidata,
    name: "Old Business Planner File",
    extensions: &["udf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
