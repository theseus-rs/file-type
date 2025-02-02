use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51756571: FileFormat = FileFormat {
    id: 51_756_571,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Slide Library",
    extensions: &["slb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
