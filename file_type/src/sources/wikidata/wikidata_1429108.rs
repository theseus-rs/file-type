use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1429108: FileFormat = FileFormat {
    id: 1_429_108,
    source_type: SourceType::Wikidata,
    name: "NZB",
    extensions: &["nzb"],
    media_types: &["application/x-nzb"],
    internal_signatures: &[],
    related_formats: &[],
};
