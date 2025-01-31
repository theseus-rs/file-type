use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113846496: FileFormat = FileFormat {
    id: 113_846_496,
    puid: "wikidata/113846496",
    name: "SureThing Template",
    extensions: &["stt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
