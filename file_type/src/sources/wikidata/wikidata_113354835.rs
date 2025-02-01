use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113354835: FileFormat = FileFormat {
    id: 113_354_835,
    puid: "wikidata/113354835",
    name: "Snagit Preset file",
    extensions: &["snagpresets"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
