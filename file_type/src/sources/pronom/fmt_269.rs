use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_269: FileFormat = FileFormat {
    id: 1_007,
    puid: "fmt/269",
    name: "Microsoft Works Database for Macintosh",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
