use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26211840: FileFormat = FileFormat {
    id: 26_211_840,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, ISO/IEC 21320–1:2015",
    extensions: &["zip"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
