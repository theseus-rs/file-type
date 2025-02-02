use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47539005: FileFormat = FileFormat {
    id: 47_539_005,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
