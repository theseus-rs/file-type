use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000603: FileFormat = FileFormat {
    id: 29_000_603,
    source_type: SourceType::Wikidata,
    name: "Windows Registry policy file",
    extensions: &["pol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
