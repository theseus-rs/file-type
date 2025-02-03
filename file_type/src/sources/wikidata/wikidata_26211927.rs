use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26211927: FileFormat = FileFormat {
    id: 26_211_927,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 6.2.0",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
