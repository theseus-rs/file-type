use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113494682: FileFormat = FileFormat {
    id: 113_494_682,
    source_type: SourceType::Wikidata,
    name: "Persuasion Auto-Template Interchange File",
    extensions: &["atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
