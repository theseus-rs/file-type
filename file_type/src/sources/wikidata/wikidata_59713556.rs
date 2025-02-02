use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59713556: FileFormat = FileFormat {
    id: 59_713_556,
    source_type: SourceType::Wikidata,
    name: "Gaussian Input Data File",
    extensions: &["gjf"],
    media_types: &["chemical/x-gaussian-input"],
    internal_signatures: &[],
    related_formats: &[],
};
