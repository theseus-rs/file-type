use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71837258: FileFormat = FileFormat {
    id: 71_837_258,
    puid: "wikidata/71837258",
    name: "CorelDraw Compressed Drawing file format",
    extensions: &["cdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
