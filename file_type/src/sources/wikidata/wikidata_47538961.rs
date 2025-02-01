use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538961: FileFormat = FileFormat {
    id: 47_538_961,
    puid: "wikidata/47538961",
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
