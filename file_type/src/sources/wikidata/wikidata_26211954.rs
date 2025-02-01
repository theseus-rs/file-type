use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211954: FileFormat = FileFormat {
    id: 26_211_954,
    puid: "wikidata/26211954",
    name: "ZIP archive file format, version 4.6",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
