use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211940: FileFormat = FileFormat {
    id: 26_211_940,
    puid: "wikidata/26211940",
    name: "ZIP archive file format, version 5.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
