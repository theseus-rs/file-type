use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211978: FileFormat = FileFormat {
    id: 26_211_978,
    puid: "wikidata/26211978",
    name: "ZIP archive file format, version 1.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
