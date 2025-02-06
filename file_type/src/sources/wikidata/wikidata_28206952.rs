use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206952: FileFormat = FileFormat {
    id: 28_206_952,
    source_type: SourceType::Wikidata,
    name: "PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
