use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112653362: FileFormat = FileFormat {
    id: 112_653_362,
    source_type: SourceType::Wikidata,
    name: "Astound Draw file",
    extensions: &["adw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
