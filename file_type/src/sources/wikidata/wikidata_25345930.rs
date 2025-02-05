use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25345930: FileFormat = FileFormat {
    id: 25_345_930,
    source_type: SourceType::Wikidata,
    name: "Citrine",
    extensions: &["ctr"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
