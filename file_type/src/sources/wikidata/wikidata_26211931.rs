use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211931: FileFormat = FileFormat {
    id: 26_211_931,
    puid: "wikidata/26211931",
    name: "ZIP archive file format, version 6.1.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
