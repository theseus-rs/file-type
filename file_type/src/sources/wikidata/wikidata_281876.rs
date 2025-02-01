use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_281876: FileFormat = FileFormat {
    id: 281_876,
    puid: "wikidata/281876",
    name: "YAML",
    extensions: &["yaml", "yml"],
    media_types: &["application/yaml", "application/yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
