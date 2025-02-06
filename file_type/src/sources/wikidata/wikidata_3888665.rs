use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3888665: FileFormat = FileFormat {
    id: 3_888_665,
    source_type: SourceType::Wikidata,
    name: "Package interchange format",
    extensions: &["pif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
