use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117276362: FileFormat = FileFormat {
    id: 117_276_362,
    puid: "wikidata/117276362",
    name: "Ultimate Business Planner File",
    extensions: &["bp1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
