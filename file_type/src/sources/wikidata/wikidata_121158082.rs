use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121158082: FileFormat = FileFormat {
    id: 121_158_082,
    source_type: SourceType::Wikidata,
    name: "ResumeMaker file",
    extensions: &["rmr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
