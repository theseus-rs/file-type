use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61752032: FileFormat = FileFormat {
    id: 61_752_032,
    puid: "wikidata/61752032",
    name: "FileMaker Pro Database, version 7",
    extensions: &["fp7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
