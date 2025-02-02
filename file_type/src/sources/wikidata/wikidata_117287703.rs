use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117287703: FileFormat = FileFormat {
    id: 117_287_703,
    source_type: SourceType::Wikidata,
    name: "SigmaStat DOS Worksheet",
    extensions: &["sp5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
