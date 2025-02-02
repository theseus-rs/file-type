use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960004: FileFormat = FileFormat {
    id: 27_960_004,
    source_type: SourceType::Wikidata,
    name: "Real Lossless Codec",
    extensions: &["rmvb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
