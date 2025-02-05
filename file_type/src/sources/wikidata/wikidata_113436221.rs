use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113436221: FileFormat = FileFormat {
    id: 113_436_221,
    source_type: SourceType::Wikidata,
    name: "OBO Flat File Format 1.2",
    extensions: &["obo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
