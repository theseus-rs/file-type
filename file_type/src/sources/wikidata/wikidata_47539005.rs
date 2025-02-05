use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539005: FileFormat = FileFormat {
    id: 47_539_005,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
