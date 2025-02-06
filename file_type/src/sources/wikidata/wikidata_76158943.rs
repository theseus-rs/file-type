use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76158943: FileFormat = FileFormat {
    id: 76_158_943,
    source_type: SourceType::Wikidata,
    name: "MegaPaint VPO",
    extensions: &["vpo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
