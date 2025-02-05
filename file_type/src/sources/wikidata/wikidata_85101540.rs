use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85101540: FileFormat = FileFormat {
    id: 85_101_540,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Project File 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
