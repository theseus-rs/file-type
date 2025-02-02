use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83794435: FileFormat = FileFormat {
    id: 83_794_435,
    source_type: SourceType::Wikidata,
    name: "EclipseCrossword Word List File",
    extensions: &["ewl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
