use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600255: FileFormat = FileFormat {
    id: 28_600_255,
    source_type: SourceType::Wikidata,
    name: "ARTS",
    extensions: &["arts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
