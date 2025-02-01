use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85013182: FileFormat = FileFormat {
    id: 85_013_182,
    puid: "wikidata/85013182",
    name: "Autodesk Revit Project File, version 2008",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
