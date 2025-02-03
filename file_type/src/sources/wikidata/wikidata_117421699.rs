use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117421699: FileFormat = FileFormat {
    id: 117_421_699,
    source_type: SourceType::Wikidata,
    name: "JSONC",
    extensions: &["jsonc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
