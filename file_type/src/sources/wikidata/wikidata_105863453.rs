use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863453: FileFormat = FileFormat {
    id: 105_863_453,
    source_type: SourceType::Wikidata,
    name: "Okuma CNC program",
    extensions: &["min"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
