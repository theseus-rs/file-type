use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951550: FileFormat = FileFormat {
    id: 126_951_550,
    source_type: SourceType::Wikidata,
    name: "J script file",
    extensions: &["ijs"],
    media_types: &["text/x-j"],
    internal_signatures: &[],
    related_formats: &[],
};
