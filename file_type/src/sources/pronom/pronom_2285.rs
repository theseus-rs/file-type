use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2285: FileFormat = FileFormat {
    id: 2_285,
    source_type: SourceType::Pronom,
    name: "Comic Book Archive",
    extensions: &["cb7", "cba", "cbr", "cbt", "cbz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
