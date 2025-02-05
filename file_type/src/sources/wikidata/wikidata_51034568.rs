use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51034568: FileFormat = FileFormat {
    id: 51_034_568,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 9",
    extensions: &["pspimage"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
