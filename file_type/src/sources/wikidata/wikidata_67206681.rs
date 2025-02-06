use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206681: FileFormat = FileFormat {
    id: 67_206_681,
    source_type: SourceType::Wikidata,
    name: "TurboCAD Template",
    extensions: &["tct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
