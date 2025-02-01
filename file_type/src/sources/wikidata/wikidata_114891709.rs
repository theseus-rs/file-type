use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114891709: FileFormat = FileFormat {
    id: 114_891_709,
    puid: "wikidata/114891709",
    name: "Tax Export File",
    extensions: &["txf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
