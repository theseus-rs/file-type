use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_193: FileFormat = FileFormat {
    id: 193,
    source_type: SourceType::Pronom,
    name: "AutoCAD Device-Independent Binary Plotter File",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
