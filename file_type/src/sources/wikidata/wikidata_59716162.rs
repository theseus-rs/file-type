use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59716162: FileFormat = FileFormat {
    id: 59_716_162,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Chart",
    extensions: &["ch3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
