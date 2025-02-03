use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27996251: FileFormat = FileFormat {
    id: 27_996_251,
    source_type: SourceType::Wikidata,
    name: "InnoDB database file",
    extensions: &["ibd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
