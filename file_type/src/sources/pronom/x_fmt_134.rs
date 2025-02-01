use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_134: FileFormat = FileFormat {
    id: 193,
    puid: "x-fmt/134",
    name: "AutoCAD Device-Independent Binary Plotter File",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
