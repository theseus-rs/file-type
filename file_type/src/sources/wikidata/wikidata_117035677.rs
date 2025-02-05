use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117035677: FileFormat = FileFormat {
    id: 117_035_677,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows Metafile",
    extensions: &["wmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
