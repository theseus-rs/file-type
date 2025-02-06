use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127427530: FileFormat = FileFormat {
    id: 127_427_530,
    source_type: SourceType::Wikidata,
    name: "GGUF",
    extensions: &["gguf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
