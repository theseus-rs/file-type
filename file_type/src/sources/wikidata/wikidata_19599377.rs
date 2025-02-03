use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_19599377: FileFormat = FileFormat {
    id: 19_599_377,
    source_type: SourceType::Wikidata,
    name: "AppleLink Package Compression Format",
    extensions: &["pkg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
