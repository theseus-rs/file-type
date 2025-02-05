use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650534: FileFormat = FileFormat {
    id: 29_650_534,
    source_type: SourceType::Wikidata,
    name: "PaintJet soft font",
    extensions: &["pjf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
