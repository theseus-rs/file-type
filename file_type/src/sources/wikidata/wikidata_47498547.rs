use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498547: FileFormat = FileFormat {
    id: 47_498_547,
    puid: "wikidata/47498547",
    name: "Adobe Illustrator file format, version 10",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
