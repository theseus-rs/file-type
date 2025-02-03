use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111648750: FileFormat = FileFormat {
    id: 111_648_750,
    source_type: SourceType::Wikidata,
    name: "Easy Prints file",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
