use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51034568: FileFormat = FileFormat {
    id: 51_034_568,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 9",
    extensions: &["pspimage"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
