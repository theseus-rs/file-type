use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863111: FileFormat = FileFormat {
    id: 27_863_111,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 1.4",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[],
    related_formats: &[],
};
