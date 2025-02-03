use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50182524: FileFormat = FileFormat {
    id: 50_182_524,
    source_type: SourceType::Wikidata,
    name: "Open Inventor File Format, v1",
    extensions: &["iv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
