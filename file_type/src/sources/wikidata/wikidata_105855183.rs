use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855183: FileFormat = FileFormat {
    id: 105_855_183,
    source_type: SourceType::Wikidata,
    name: "LaTeX Font Definition (with rem)",
    extensions: &["fd"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
