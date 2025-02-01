use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_870: FileFormat = FileFormat {
    id: 1_674,
    puid: "fmt/870",
    name: "Perl Script",
    extensions: &["pl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
