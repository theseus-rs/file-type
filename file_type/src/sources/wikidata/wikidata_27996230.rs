use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996230: FileFormat = FileFormat {
    id: 27_996_230,
    puid: "wikidata/27996230",
    name: "FileMaker Pro Database, version 5",
    extensions: &["fp5"],
    media_types: &["application/x-filemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
