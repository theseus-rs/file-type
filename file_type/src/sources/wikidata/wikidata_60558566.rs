use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60558566: FileFormat = FileFormat {
    id: 60_558_566,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Word Processor",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
