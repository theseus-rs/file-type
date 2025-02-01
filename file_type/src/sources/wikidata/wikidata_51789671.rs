use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51789671: FileFormat = FileFormat {
    id: 51_789_671,
    puid: "wikidata/51789671",
    name: "AutoCAD External Database Configuration File",
    extensions: &["udl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
