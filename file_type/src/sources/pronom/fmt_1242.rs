use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1242: FileFormat = FileFormat {
    id: 2_060,
    puid: "fmt/1242",
    name: "ZFO (Form) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    internal_signatures: &[],
    related_formats: &[],
};
