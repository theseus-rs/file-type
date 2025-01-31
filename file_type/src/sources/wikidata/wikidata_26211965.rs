use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211965: FileFormat = FileFormat {
    id: 26_211_965,
    puid: "wikidata/26211965",
    name: "ZIP archive file format, version 2.5",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
