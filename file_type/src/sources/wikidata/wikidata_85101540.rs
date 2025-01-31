use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85101540: FileFormat = FileFormat {
    id: 85_101_540,
    puid: "wikidata/85101540",
    name: "Autodesk Revit Project File 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
