use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206465: FileFormat = FileFormat {
    id: 28_206_465,
    puid: "wikidata/28206465",
    name: "KoalaPainter uncompressed",
    extensions: &["koa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
