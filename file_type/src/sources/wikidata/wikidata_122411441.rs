use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122411441: FileFormat = FileFormat {
    id: 122_411_441,
    puid: "wikidata/122411441",
    name: "DWARF Symbolic File",
    extensions: &["dwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
