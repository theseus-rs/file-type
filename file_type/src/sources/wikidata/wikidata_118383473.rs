use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118383473: FileFormat = FileFormat {
    id: 118_383_473,
    puid: "wikidata/118383473",
    name: "Album Book file",
    extensions: &["opf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
