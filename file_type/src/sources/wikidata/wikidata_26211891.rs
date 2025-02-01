use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211891: FileFormat = FileFormat {
    id: 26_211_891,
    puid: "wikidata/26211891",
    name: "ZIP archive file format, version 6.3.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
