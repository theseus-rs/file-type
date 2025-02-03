use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600493: FileFormat = FileFormat {
    id: 28_600_493,
    source_type: SourceType::Wikidata,
    name: "DeltaVision",
    extensions: &["dv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
