use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000603: FileFormat = FileFormat {
    id: 29_000_603,
    puid: "wikidata/29000603",
    name: "Windows Registry policy file",
    extensions: &["pol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
