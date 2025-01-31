use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21040799: FileFormat = FileFormat {
    id: 21_040_799,
    puid: "wikidata/21040799",
    name: "MadTracker 2 format",
    extensions: &["mt2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
