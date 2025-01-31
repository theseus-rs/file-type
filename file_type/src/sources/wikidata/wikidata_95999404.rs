use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95999404: FileFormat = FileFormat {
    id: 95_999_404,
    puid: "wikidata/95999404",
    name: "Graph6",
    extensions: &["g6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
