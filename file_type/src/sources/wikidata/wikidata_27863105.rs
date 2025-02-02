use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863105: FileFormat = FileFormat {
    id: 27_863_105,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 1.0",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
