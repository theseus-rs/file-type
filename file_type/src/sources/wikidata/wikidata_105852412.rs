use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852412: FileFormat = FileFormat {
    id: 105_852_412,
    source_type: SourceType::Wikidata,
    name: "SExtractor configuration",
    extensions: &["sex"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
