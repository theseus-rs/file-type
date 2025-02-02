use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121158405: FileFormat = FileFormat {
    id: 121_158_405,
    source_type: SourceType::Wikidata,
    name: "Merge file",
    extensions: &["mrg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
