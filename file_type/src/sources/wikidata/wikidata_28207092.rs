use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207092: FileFormat = FileFormat {
    id: 28_207_092,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Index file",
    extensions: &["sdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
