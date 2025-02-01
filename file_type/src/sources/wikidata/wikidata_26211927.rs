use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211927: FileFormat = FileFormat {
    id: 26_211_927,
    puid: "wikidata/26211927",
    name: "ZIP archive file format, version 6.2.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
