use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1701: FileFormat = FileFormat {
    id: 1_701,
    source_type: SourceType::Pronom,
    name: "Compressed MusicXML",
    extensions: &["mxl"],
    media_types: &["application/vnd.recordare.musicxml"],
    signatures: &[],
    related_formats: &[],
};
