use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128694058: FileFormat = FileFormat {
    id: 128_694_058,
    source_type: SourceType::Wikidata,
    name: "OpenBugs file",
    extensions: &["bug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
