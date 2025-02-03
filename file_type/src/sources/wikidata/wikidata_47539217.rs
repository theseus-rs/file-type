use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47539217: FileFormat = FileFormat {
    id: 47_539_217,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Batch Plot File, version 1.0-R14",
    extensions: &["bp2", "bpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
