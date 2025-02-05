use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131466271: FileFormat = FileFormat {
    id: 131_466_271,
    source_type: SourceType::Wikidata,
    name: "Guy’s Image Processing Lab file format",
    extensions: &["gipl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
