use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116647633: FileFormat = FileFormat {
    id: 116_647_633,
    source_type: SourceType::Wikidata,
    name: "KeyForm 3.x Document",
    extensions: &["ifd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
