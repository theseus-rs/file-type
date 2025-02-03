use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600399: FileFormat = FileFormat {
    id: 28_600_399,
    source_type: SourceType::Wikidata,
    name: "Arma PBO",
    extensions: &["pbo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
