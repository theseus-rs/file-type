use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85101540: FileFormat = FileFormat {
    id: 85_101_540,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Project File 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
