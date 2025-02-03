use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26211915: FileFormat = FileFormat {
    id: 26_211_915,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 6.2.1",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
