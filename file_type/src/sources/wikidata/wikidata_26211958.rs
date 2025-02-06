use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26211958: FileFormat = FileFormat {
    id: 26_211_958,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 2.7",
    extensions: &["zip"],
    media_types: &["application/zip"],
    signatures: &[],
    related_formats: &[],
};
