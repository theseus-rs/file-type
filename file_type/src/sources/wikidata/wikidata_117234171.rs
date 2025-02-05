use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117234171: FileFormat = FileFormat {
    id: 117_234_171,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows 3D Model File",
    extensions: &["mdi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
