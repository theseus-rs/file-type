use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211975: FileFormat = FileFormat {
    id: 26_211_975,
    puid: "wikidata/26211975",
    name: "ZIP archive file format, version 2.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
