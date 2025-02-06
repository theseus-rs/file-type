use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110535991: FileFormat = FileFormat {
    id: 110_535_991,
    source_type: SourceType::Wikidata,
    name: "Movie Magic Screenwriter backup document",
    extensions: &["bk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
