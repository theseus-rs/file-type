use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855183: FileFormat = FileFormat {
    id: 105_855_183,
    puid: "wikidata/105855183",
    name: "LaTeX Font Definition (with rem)",
    extensions: &["fd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
