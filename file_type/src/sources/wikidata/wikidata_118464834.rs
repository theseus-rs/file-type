use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118464834: FileFormat = FileFormat {
    id: 118_464_834,
    source_type: SourceType::Wikidata,
    name: "Enhanced Image Package",
    extensions: &["eip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
