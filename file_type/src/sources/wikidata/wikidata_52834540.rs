use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52834540: FileFormat = FileFormat {
    id: 52_834_540,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 4",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
