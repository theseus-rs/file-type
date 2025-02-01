use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34303434: FileFormat = FileFormat {
    id: 34_303_434,
    puid: "wikidata/34303434",
    name: "SYSDOOM script",
    extensions: &["doom"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
