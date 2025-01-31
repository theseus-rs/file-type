use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498538: FileFormat = FileFormat {
    id: 47_498_538,
    puid: "wikidata/47498538",
    name: "Adobe Illustrator file format, version 9.0",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
