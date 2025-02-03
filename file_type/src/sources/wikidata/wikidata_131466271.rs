use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131466271: FileFormat = FileFormat {
    id: 131_466_271,
    source_type: SourceType::Wikidata,
    name: "Guy’s Image Processing Lab file format",
    extensions: &["gipl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
