use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967445: FileFormat = FileFormat {
    id: 27_967_445,
    puid: "wikidata/27967445",
    name: "Autodesk Animator Pro FLIC",
    extensions: &["flc"],
    media_types: &["video/flc"],
    internal_signatures: &[],
    related_formats: &[],
};
