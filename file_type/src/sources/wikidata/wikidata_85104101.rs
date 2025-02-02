use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85104101: FileFormat = FileFormat {
    id: 85_104_101,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Family File, version 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
