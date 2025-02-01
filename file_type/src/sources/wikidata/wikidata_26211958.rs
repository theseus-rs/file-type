use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211958: FileFormat = FileFormat {
    id: 26_211_958,
    puid: "wikidata/26211958",
    name: "ZIP archive file format, version 2.7",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
