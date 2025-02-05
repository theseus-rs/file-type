use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26211948: FileFormat = FileFormat {
    id: 26_211_948,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 5.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
