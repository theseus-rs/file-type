use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967143: FileFormat = FileFormat {
    id: 27_967_143,
    source_type: SourceType::Wikidata,
    name: "DigiTrekker module",
    extensions: &["dtm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
