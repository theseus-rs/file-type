use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967143: FileFormat = FileFormat {
    id: 27_967_143,
    source_type: SourceType::Wikidata,
    name: "DigiTrekker module",
    extensions: &["dtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
