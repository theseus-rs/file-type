use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17144293: FileFormat = FileFormat {
    id: 17_144_293,
    source_type: SourceType::Wikidata,
    name: "UBJSON",
    extensions: &["ubj"],
    media_types: &["application/ubjson"],
    internal_signatures: &[],
    related_formats: &[],
};
