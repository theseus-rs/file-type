use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852412: FileFormat = FileFormat {
    id: 105_852_412,
    source_type: SourceType::Wikidata,
    name: "SExtractor configuration",
    extensions: &["sex"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
