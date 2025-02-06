use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85104101: FileFormat = FileFormat {
    id: 85_104_101,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Family File, version 2019",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
