use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84997326: FileFormat = FileFormat {
    id: 84_997_326,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit File, version 4",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
