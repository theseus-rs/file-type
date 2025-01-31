use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50604550: FileFormat = FileFormat {
    id: 50_604_550,
    puid: "wikidata/50604550",
    name: "FileMaker Pro Database, version 2",
    extensions: &["fm"],
    media_types: &["application/x-filemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
