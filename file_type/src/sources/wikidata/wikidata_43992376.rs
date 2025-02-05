use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43992376: FileFormat = FileFormat {
    id: 43_992_376,
    source_type: SourceType::Wikidata,
    name: "ABIF file format",
    extensions: &["abif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
