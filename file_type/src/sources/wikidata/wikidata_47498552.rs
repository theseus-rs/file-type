use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498552: FileFormat = FileFormat {
    id: 47_498_552,
    puid: "wikidata/47498552",
    name: "Adobe Illustrator file format, version 11",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
