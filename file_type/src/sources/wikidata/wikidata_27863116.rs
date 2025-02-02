use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863116: FileFormat = FileFormat {
    id: 27_863_116,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2.1",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
