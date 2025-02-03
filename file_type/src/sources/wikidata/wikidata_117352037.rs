use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117352037: FileFormat = FileFormat {
    id: 117_352_037,
    source_type: SourceType::Wikidata,
    name: "OrCAD project",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
