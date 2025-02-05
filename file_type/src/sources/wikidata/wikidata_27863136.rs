use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863136: FileFormat = FileFormat {
    id: 27_863_136,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2007-2008",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[],
    related_formats: &[],
};
