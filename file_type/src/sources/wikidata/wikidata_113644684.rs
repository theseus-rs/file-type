use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113644684: FileFormat = FileFormat {
    id: 113_644_684,
    source_type: SourceType::Wikidata,
    name: "Ulead File For Photo Projects",
    extensions: &["ufp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
