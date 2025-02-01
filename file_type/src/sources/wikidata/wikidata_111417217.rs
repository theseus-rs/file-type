use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417217: FileFormat = FileFormat {
    id: 111_417_217,
    puid: "wikidata/111417217",
    name: "Assembly Language Source Code File",
    extensions: &["asm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
