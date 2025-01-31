use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954279: FileFormat = FileFormat {
    id: 51_954_279,
    puid: "wikidata/51954279",
    name: "Autodesk Animator CEL File Format",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
