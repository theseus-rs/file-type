use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67206681: FileFormat = FileFormat {
    id: 67_206_681,
    source_type: SourceType::Wikidata,
    name: "TurboCAD Template",
    extensions: &["tct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
