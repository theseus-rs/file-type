use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960007: FileFormat = FileFormat {
    id: 27_960_007,
    source_type: SourceType::Wikidata,
    name: "RK Audio",
    extensions: &["rka"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
