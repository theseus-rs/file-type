use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904619: FileFormat = FileFormat {
    id: 29_904_619,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System access descriptor file",
    extensions: &["sa2", "sa7", "sas7bacs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
