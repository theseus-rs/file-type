use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539001: FileFormat = FileFormat {
    id: 47_539_001,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Linetype Definition File",
    extensions: &["lin"],
    media_types: &["application/x-autocad"],
    signatures: &[],
    related_formats: &[],
};
