use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600399: FileFormat = FileFormat {
    id: 28_600_399,
    source_type: SourceType::Wikidata,
    name: "Arma PBO",
    extensions: &["pbo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
