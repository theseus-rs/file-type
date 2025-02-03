use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71999956: FileFormat = FileFormat {
    id: 71_999_956,
    source_type: SourceType::Wikidata,
    name: "iTunes Cover Flow Data file format, version 2",
    extensions: &["itc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
