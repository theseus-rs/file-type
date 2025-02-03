use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113436221: FileFormat = FileFormat {
    id: 113_436_221,
    source_type: SourceType::Wikidata,
    name: "OBO Flat File Format 1.2",
    extensions: &["obo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
