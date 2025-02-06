use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206152: FileFormat = FileFormat {
    id: 28_206_152,
    source_type: SourceType::Wikidata,
    name: "FSH",
    extensions: &["fsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
