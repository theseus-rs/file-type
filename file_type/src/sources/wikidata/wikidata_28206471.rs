use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206471: FileFormat = FileFormat {
    id: 28_206_471,
    source_type: SourceType::Wikidata,
    name: "KoalaPainter compressed",
    extensions: &["gg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
