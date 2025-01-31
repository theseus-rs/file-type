use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498749: FileFormat = FileFormat {
    id: 47_498_749,
    puid: "wikidata/47498749",
    name: "Adobe Illustrator file format, version 16",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
