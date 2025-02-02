use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863143: FileFormat = FileFormat {
    id: 27_863_143,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2013-2014",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
