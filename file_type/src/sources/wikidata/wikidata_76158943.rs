use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76158943: FileFormat = FileFormat {
    id: 76_158_943,
    source_type: SourceType::Wikidata,
    name: "MegaPaint VPO",
    extensions: &["vpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
