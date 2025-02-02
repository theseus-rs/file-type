use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009602: FileFormat = FileFormat {
    id: 111_009_602,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Business Card File format",
    extensions: &["biz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
