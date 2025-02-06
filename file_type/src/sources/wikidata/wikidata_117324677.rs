use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117324677: FileFormat = FileFormat {
    id: 117_324_677,
    source_type: SourceType::Wikidata,
    name: "User Interface file",
    extensions: &["uir"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
