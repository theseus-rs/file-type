use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_11231091: FileFormat = FileFormat {
    id: 11_231_091,
    source_type: SourceType::Wikidata,
    name: "MASL",
    extensions: &["Msl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
