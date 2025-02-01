use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905354: FileFormat = FileFormat {
    id: 29_905_354,
    puid: "wikidata/29905354",
    name: "Self-contained Information Retention Format",
    extensions: &["json", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
