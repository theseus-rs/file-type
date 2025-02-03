use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7060634: FileFormat = FileFormat {
    id: 7_060_634,
    source_type: SourceType::Wikidata,
    name: "Norton Guides",
    extensions: &["ng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
