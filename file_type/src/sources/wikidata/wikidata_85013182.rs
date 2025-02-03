use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85013182: FileFormat = FileFormat {
    id: 85_013_182,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Project File, version 2008",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
