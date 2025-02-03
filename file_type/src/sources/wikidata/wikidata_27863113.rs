use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863113: FileFormat = FileFormat {
    id: 27_863_113,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2.0",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
