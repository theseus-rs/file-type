use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863134: FileFormat = FileFormat {
    id: 27_863_134,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2004-2005",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[],
    related_formats: &[],
};
