use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211983: FileFormat = FileFormat {
    id: 26_211_983,
    puid: "wikidata/26211983",
    name: "ZIP archive file format, version 1.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
