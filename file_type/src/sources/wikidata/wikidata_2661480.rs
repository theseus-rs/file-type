use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2661480: FileFormat = FileFormat {
    id: 2_661_480,
    source_type: SourceType::Wikidata,
    name: "BSON",
    extensions: &["bson"],
    media_types: &["application/bson"],
    signatures: &[],
    related_formats: &[],
};
