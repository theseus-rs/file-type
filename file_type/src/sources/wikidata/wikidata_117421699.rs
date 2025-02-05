use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117421699: FileFormat = FileFormat {
    id: 117_421_699,
    source_type: SourceType::Wikidata,
    name: "JSONC",
    extensions: &["jsonc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
