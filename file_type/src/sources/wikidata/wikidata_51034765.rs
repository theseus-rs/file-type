use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51034765: FileFormat = FileFormat {
    id: 51_034_765,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 10",
    extensions: &["pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
