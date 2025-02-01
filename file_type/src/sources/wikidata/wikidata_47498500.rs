use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47498500: FileFormat = FileFormat {
    id: 47_498_500,
    puid: "wikidata/47498500",
    name: "Adobe Illustrator file format, version 8",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
