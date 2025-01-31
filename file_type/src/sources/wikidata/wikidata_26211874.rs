use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211874: FileFormat = FileFormat {
    id: 26_211_874,
    puid: "wikidata/26211874",
    name: "ZIP archive file format, version 6.3.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
