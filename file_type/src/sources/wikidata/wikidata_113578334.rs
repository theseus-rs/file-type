use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113578334: FileFormat = FileFormat {
    id: 113_578_334,
    source_type: SourceType::Wikidata,
    name: "Music Maker Arrangement File",
    extensions: &["mmm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
