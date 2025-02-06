use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206465: FileFormat = FileFormat {
    id: 28_206_465,
    source_type: SourceType::Wikidata,
    name: "KoalaPainter uncompressed",
    extensions: &["koa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
