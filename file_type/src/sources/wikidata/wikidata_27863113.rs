use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863113: FileFormat = FileFormat {
    id: 27_863_113,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2.0",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[],
    related_formats: &[],
};
