use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_382011: FileFormat = FileFormat {
    id: 382_011,
    source_type: SourceType::Wikidata,
    name: "Program information file",
    extensions: &["pif"],
    media_types: &["application/x-pif"],
    internal_signatures: &[],
    related_formats: &[],
};
