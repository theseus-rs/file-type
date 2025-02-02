use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49619991: FileFormat = FileFormat {
    id: 49_619_991,
    source_type: SourceType::Wikidata,
    name: "Revit External Group",
    extensions: &["rvg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
