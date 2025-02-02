use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121157531: FileFormat = FileFormat {
    id: 121_157_531,
    source_type: SourceType::Wikidata,
    name: "FloorPlan 3D Template",
    extensions: &["fpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
