use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51756571: FileFormat = FileFormat {
    id: 51_756_571,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Slide Library",
    extensions: &["slb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
