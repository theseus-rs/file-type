use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117871620: FileFormat = FileFormat {
    id: 117_871_620,
    puid: "wikidata/117871620",
    name: "U.S. Robotics WorldPort 2496 file",
    extensions: &["wpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
