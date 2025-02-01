use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866050: FileFormat = FileFormat {
    id: 105_866_050,
    puid: "wikidata/105866050",
    name: "Planner project",
    extensions: &["planner"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
