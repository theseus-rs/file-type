use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85027567: FileFormat = FileFormat {
    id: 85_027_567,
    source_type: SourceType::Wikidata,
    name: "Autodesk Revit Family File, version 2008",
    extensions: &["rfa", "rft", "rte", "rvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
