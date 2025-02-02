use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50413749: FileFormat = FileFormat {
    id: 50_413_749,
    source_type: SourceType::Wikidata,
    name: "Lightwright 4 Show File",
    extensions: &["lw4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
