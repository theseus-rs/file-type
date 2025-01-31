use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113435494: FileFormat = FileFormat {
    id: 113_435_494,
    puid: "wikidata/113435494",
    name: "Garmin track log file",
    extensions: &["gmn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
