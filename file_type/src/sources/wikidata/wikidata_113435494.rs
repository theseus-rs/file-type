use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113435494: FileFormat = FileFormat {
    id: 113_435_494,
    source_type: SourceType::Wikidata,
    name: "Garmin track log file",
    extensions: &["gmn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
