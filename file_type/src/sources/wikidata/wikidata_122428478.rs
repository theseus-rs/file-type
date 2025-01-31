use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122428478: FileFormat = FileFormat {
    id: 122_428_478,
    puid: "wikidata/122428478",
    name: "Wild Photo Effects file",
    extensions: &["moo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
