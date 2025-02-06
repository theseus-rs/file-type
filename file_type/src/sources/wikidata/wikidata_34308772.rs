use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34308772: FileFormat = FileFormat {
    id: 34_308_772,
    source_type: SourceType::Wikidata,
    name: "Scrivener document",
    extensions: &["scriv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
