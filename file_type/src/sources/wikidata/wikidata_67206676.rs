use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206676: FileFormat = FileFormat {
    id: 67_206_676,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows Drawing file",
    extensions: &["tcw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
