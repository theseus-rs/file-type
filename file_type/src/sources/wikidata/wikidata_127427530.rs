use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127427530: FileFormat = FileFormat {
    id: 127_427_530,
    source_type: SourceType::Wikidata,
    name: "GGUF",
    extensions: &["gguf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
