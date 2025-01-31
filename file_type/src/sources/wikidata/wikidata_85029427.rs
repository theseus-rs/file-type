use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85029427: FileFormat = FileFormat {
    id: 85_029_427,
    puid: "wikidata/85029427",
    name: "Autodesk Revit Family File, version 2010",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
