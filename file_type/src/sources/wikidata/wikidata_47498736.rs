use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498736: FileFormat = FileFormat {
    id: 47_498_736,
    puid: "wikidata/47498736",
    name: "Adobe Illustrator file format, version 14",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
