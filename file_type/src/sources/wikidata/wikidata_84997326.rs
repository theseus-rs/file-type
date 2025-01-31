use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84997326: FileFormat = FileFormat {
    id: 84_997_326,
    puid: "wikidata/84997326",
    name: "Autodesk Revit File, version 4",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
