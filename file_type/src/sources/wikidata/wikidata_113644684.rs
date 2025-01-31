use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113644684: FileFormat = FileFormat {
    id: 113_644_684,
    puid: "wikidata/113644684",
    name: "Ulead File For Photo Projects",
    extensions: &["ufp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
