use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122408089: FileFormat = FileFormat {
    id: 122_408_089,
    puid: "wikidata/122408089",
    name: "PlayStation Debug Executable",
    extensions: &["pse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
