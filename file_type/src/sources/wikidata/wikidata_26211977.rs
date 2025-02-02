use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26211977: FileFormat = FileFormat {
    id: 26_211_977,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 2.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
