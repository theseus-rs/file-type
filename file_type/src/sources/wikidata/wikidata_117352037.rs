use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117352037: FileFormat = FileFormat {
    id: 117_352_037,
    source_type: SourceType::Wikidata,
    name: "OrCAD project",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
