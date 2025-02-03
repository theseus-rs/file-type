use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27863110: FileFormat = FileFormat {
    id: 27_863_110,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 1.3",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
