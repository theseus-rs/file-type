use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206465: FileFormat = FileFormat {
    id: 28_206_465,
    source_type: SourceType::Wikidata,
    name: "KoalaPainter uncompressed",
    extensions: &["koa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
