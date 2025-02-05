use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7060634: FileFormat = FileFormat {
    id: 7_060_634,
    source_type: SourceType::Wikidata,
    name: "Norton Guides",
    extensions: &["ng"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
