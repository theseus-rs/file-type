use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117842873: FileFormat = FileFormat {
    id: 117_842_873,
    source_type: SourceType::Wikidata,
    name: "EDMICS 6",
    extensions: &["ed6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
