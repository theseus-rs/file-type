use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52060199: FileFormat = FileFormat {
    id: 52_060_199,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 7",
    extensions: &["psp", "pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
