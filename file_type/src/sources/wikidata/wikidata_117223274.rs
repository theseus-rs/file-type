use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117223274: FileFormat = FileFormat {
    id: 117_223_274,
    puid: "wikidata/117223274",
    name: "LDB File",
    extensions: &["ldb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
