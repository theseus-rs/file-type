use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49244284: FileFormat = FileFormat {
    id: 49_244_284,
    source_type: SourceType::Wikidata,
    name: "form*Z Project File",
    extensions: &["fmz"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
