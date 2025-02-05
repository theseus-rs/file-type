use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116878054: FileFormat = FileFormat {
    id: 116_878_054,
    source_type: SourceType::Wikidata,
    name: "Address Book Text",
    extensions: &["AB5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
