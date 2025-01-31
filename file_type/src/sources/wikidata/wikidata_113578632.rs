use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113578632: FileFormat = FileFormat {
    id: 113_578_632,
    puid: "wikidata/113578632",
    name: "MAGIX photo album",
    extensions: &["alb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
