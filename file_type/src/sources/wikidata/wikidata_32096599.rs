use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32096599: FileFormat = FileFormat {
    id: 32_096_599,
    source_type: SourceType::Wikidata,
    name: "Gran Turismo File System",
    extensions: &["vol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
