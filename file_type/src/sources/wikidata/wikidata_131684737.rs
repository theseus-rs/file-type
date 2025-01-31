use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131684737: FileFormat = FileFormat {
    id: 131_684_737,
    puid: "wikidata/131684737",
    name: "Movie.BYU file format",
    extensions: &["g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
