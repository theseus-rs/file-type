use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72274847: FileFormat = FileFormat {
    id: 72_274_847,
    source_type: SourceType::Wikidata,
    name: "Maestro molecular model",
    extensions: &["mae"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
