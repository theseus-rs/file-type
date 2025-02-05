use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85027567: FileFormat = FileFormat {
    id: 85_027_567,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Family File, version 2008",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
