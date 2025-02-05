use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117835020: FileFormat = FileFormat {
    id: 117_835_020,
    source_type: SourceType::Wikidata,
    name: "Canon Navigator Fax file",
    extensions: &["can"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
