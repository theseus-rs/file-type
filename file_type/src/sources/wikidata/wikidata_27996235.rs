use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996235: FileFormat = FileFormat {
    id: 27_996_235,
    puid: "wikidata/27996235",
    name: "FileMaker Pro Database, version 3",
    extensions: &["fp3"],
    media_types: &["application/x-filemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
