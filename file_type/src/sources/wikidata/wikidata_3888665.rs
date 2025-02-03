use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3888665: FileFormat = FileFormat {
    id: 3_888_665,
    source_type: SourceType::Wikidata,
    name: "Package interchange format",
    extensions: &["pif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
