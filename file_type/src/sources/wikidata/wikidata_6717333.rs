use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6717333: FileFormat = FileFormat {
    id: 6_717_333,
    source_type: SourceType::Wikidata,
    name: "Mathematical Programming System",
    extensions: &["mps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
