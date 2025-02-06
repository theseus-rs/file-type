use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2060: FileFormat = FileFormat {
    id: 2_060,
    source_type: SourceType::Pronom,
    name: "ZFO (Form) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    signatures: &[],
    related_formats: &[],
};
