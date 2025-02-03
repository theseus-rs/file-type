use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967225: FileFormat = FileFormat {
    id: 27_967_225,
    source_type: SourceType::Wikidata,
    name: "D-Lusion Music File",
    extensions: &["dmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
