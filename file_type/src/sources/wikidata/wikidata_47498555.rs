use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498555: FileFormat = FileFormat {
    id: 47_498_555,
    puid: "wikidata/47498555",
    name: "Adobe Illustrator file format, version 12",
    extensions: &["ai", "pdf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
