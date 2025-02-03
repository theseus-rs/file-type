use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650534: FileFormat = FileFormat {
    id: 29_650_534,
    source_type: SourceType::Wikidata,
    name: "PaintJet soft font",
    extensions: &["pjf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
