use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113494682: FileFormat = FileFormat {
    id: 113_494_682,
    source_type: SourceType::Wikidata,
    name: "Persuasion Auto-Template Interchange File",
    extensions: &["atf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
