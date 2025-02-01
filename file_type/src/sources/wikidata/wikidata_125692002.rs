use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692002: FileFormat = FileFormat {
    id: 125_692_002,
    puid: "wikidata/125692002",
    name: "OpenDocument Graphic Template",
    extensions: &["otg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
