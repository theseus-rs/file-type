use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117234171: FileFormat = FileFormat {
    id: 117_234_171,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows 3D Model File",
    extensions: &["mdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
