use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_2005: FileFormat = FileFormat {
    id: 2_880,
    puid: "fmt/2005",
    name: "Compressed MusicXML",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    internal_signatures: &[],
    related_formats: &[],
};
