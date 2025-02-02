use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113578334: FileFormat = FileFormat {
    id: 113_578_334,
    source_type: SourceType::Wikidata,
    name: "Music Maker Arrangement File",
    extensions: &["mmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
