use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121158082: FileFormat = FileFormat {
    id: 121_158_082,
    source_type: SourceType::Wikidata,
    name: "ResumeMaker file",
    extensions: &["rmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
