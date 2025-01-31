use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25824045: FileFormat = FileFormat {
    id: 25_824_045,
    puid: "wikidata/25824045",
    name: "OSM Note File",
    extensions: &["osn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
