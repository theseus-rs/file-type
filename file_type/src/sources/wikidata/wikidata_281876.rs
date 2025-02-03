use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_281876: FileFormat = FileFormat {
    id: 281_876,
    source_type: SourceType::Wikidata,
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &["application/yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
