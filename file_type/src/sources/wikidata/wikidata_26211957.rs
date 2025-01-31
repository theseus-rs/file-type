use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211957: FileFormat = FileFormat {
    id: 26_211_957,
    puid: "wikidata/26211957",
    name: "ZIP archive file format, version 4.5",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
