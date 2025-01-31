use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211905: FileFormat = FileFormat {
    id: 26_211_905,
    puid: "wikidata/26211905",
    name: "ZIP archive file format, version 6.2.2",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
