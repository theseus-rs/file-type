use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131684737: FileFormat = FileFormat {
    id: 131_684_737,
    source_type: SourceType::Wikidata,
    name: "Movie.BYU file format",
    extensions: &["g"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
