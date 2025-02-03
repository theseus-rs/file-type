use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5448342: FileFormat = FileFormat {
    id: 5_448_342,
    source_type: SourceType::Wikidata,
    name: "File change log",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
