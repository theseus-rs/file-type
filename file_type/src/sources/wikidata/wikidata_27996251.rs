use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27996251: FileFormat = FileFormat {
    id: 27_996_251,
    source_type: SourceType::Wikidata,
    name: "InnoDB database file",
    extensions: &["ibd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
