use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_18245359: FileFormat = FileFormat {
    id: 18_245_359,
    puid: "wikidata/18245359",
    name: "Control Panel Applet",
    extensions: &["cpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
