use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498565: FileFormat = FileFormat {
    id: 47_498_565,
    puid: "wikidata/47498565",
    name: "Adobe Illustrator file format, version 13",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
