use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211948: FileFormat = FileFormat {
    id: 26_211_948,
    puid: "wikidata/26211948",
    name: "ZIP archive file format, version 5.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
