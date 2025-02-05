use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111648750: FileFormat = FileFormat {
    id: 111_648_750,
    source_type: SourceType::Wikidata,
    name: "Easy Prints file",
    extensions: &["php"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
