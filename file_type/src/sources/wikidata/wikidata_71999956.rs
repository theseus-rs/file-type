use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71999956: FileFormat = FileFormat {
    id: 71_999_956,
    source_type: SourceType::Wikidata,
    name: "iTunes Cover Flow Data file format, version 2",
    extensions: &["itc2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
