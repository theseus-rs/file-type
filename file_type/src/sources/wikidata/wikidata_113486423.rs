use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113486423: FileFormat = FileFormat {
    id: 113_486_423,
    source_type: SourceType::Wikidata,
    name: "Persuasion Mac Document 1.0",
    extensions: &["pr1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
