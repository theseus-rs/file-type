use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117814506: FileFormat = FileFormat {
    id: 117_814_506,
    source_type: SourceType::Wikidata,
    name: "Adaptive Information Systems",
    extensions: &["ais"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
