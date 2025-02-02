use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52060012: FileFormat = FileFormat {
    id: 52_060_012,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 6",
    extensions: &["psp", "pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
