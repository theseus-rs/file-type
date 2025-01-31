use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85104101: FileFormat = FileFormat {
    id: 85_104_101,
    puid: "wikidata/85104101",
    name: "Autodesk Revit Family File, version 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
