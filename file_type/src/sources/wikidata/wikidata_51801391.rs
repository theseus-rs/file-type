use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51801391: FileFormat = FileFormat {
    id: 51_801_391,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Xref Log",
    extensions: &["xlg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
