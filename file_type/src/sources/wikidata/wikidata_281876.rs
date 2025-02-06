use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_281876: FileFormat = FileFormat {
    id: 281_876,
    source_type: SourceType::Wikidata,
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &["application/yaml"],
    signatures: &[],
    related_formats: &[],
};
