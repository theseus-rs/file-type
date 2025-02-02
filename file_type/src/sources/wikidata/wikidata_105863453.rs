use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863453: FileFormat = FileFormat {
    id: 105_863_453,
    source_type: SourceType::Wikidata,
    name: "Okuma CNC program",
    extensions: &["min"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
