use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211915: FileFormat = FileFormat {
    id: 26_211_915,
    puid: "wikidata/26211915",
    name: "ZIP archive file format, version 6.2.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
