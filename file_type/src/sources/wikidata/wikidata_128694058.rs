use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128694058: FileFormat = FileFormat {
    id: 128_694_058,
    source_type: SourceType::Wikidata,
    name: "OpenBugs file",
    extensions: &["bug"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
