use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85029427: FileFormat = FileFormat {
    id: 85_029_427,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Family File, version 2010",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
