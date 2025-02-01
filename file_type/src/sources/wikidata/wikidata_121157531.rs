use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121157531: FileFormat = FileFormat {
    id: 121_157_531,
    puid: "wikidata/121157531",
    name: "FloorPlan 3D Template",
    extensions: &["fpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
