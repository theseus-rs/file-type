use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206471: FileFormat = FileFormat {
    id: 28_206_471,
    source_type: SourceType::Wikidata,
    name: "KoalaPainter compressed",
    extensions: &["gg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
