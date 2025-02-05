use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116647633: FileFormat = FileFormat {
    id: 116_647_633,
    source_type: SourceType::Wikidata,
    name: "KeyForm 3.x Document",
    extensions: &["ifd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
