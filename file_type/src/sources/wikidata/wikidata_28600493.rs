use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600493: FileFormat = FileFormat {
    id: 28_600_493,
    source_type: SourceType::Wikidata,
    name: "DeltaVision",
    extensions: &["dv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
