use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96147075: FileFormat = FileFormat {
    id: 96_147_075,
    source_type: SourceType::Wikidata,
    name: "Wolfram machine learning format",
    extensions: &["wmlf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
