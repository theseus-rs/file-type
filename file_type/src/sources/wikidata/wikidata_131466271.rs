use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131466271: FileFormat = FileFormat {
    id: 131_466_271,
    puid: "wikidata/131466271",
    name: "Guy’s Image Processing Lab file format",
    extensions: &["gipl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
