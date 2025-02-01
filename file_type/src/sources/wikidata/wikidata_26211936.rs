use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211936: FileFormat = FileFormat {
    id: 26_211_936,
    puid: "wikidata/26211936",
    name: "ZIP archive file format, version 5.2",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
