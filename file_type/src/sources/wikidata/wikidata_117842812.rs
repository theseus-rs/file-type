use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117842812: FileFormat = FileFormat {
    id: 117_842_812,
    source_type: SourceType::Wikidata,
    name: "EDMICS 5",
    extensions: &["ed5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
