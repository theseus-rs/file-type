use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049670: FileFormat = FileFormat {
    id: 28_049_670,
    puid: "wikidata/28049670",
    name: "Autodesk 3D Studio ASCII format",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
