use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121157531: FileFormat = FileFormat {
    id: 121_157_531,
    source_type: SourceType::Wikidata,
    name: "FloorPlan 3D Template",
    extensions: &["fpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
