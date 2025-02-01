use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26211840: FileFormat = FileFormat {
    id: 26_211_840,
    puid: "wikidata/26211840",
    name: "ZIP archive file format, ISO/IEC 21320–1:2015",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
