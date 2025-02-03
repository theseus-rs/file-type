use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113486423: FileFormat = FileFormat {
    id: 113_486_423,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 1.0",
    extensions: &["pr1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
