use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50413749: FileFormat = FileFormat {
    id: 50_413_749,
    source_type: SourceType::Wikidata,
    name: "Lightwright 4 Show File",
    extensions: &["lw4"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
