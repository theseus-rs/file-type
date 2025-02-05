use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112653362: FileFormat = FileFormat {
    id: 112_653_362,
    source_type: SourceType::Wikidata,
    name: "Astound Draw file",
    extensions: &["adw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
