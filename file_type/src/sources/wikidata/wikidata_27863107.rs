use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863107: FileFormat = FileFormat {
    id: 27_863_107,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 1.2",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[],
    related_formats: &[],
};
