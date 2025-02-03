use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207212: FileFormat = FileFormat {
    id: 28_207_212,
    source_type: SourceType::Wikidata,
    name: "Quantel VPB image",
    extensions: &["vpb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
