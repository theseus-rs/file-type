use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26211954: FileFormat = FileFormat {
    id: 26_211_954,
    source_type: SourceType::Wikidata,
    name: "ZIP archive file format, version 4.6",
    extensions: &["zip"],
    media_types: &["application/zip"],
    internal_signatures: &[],
    related_formats: &[],
};
