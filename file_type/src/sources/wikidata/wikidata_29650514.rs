use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650514: FileFormat = FileFormat {
    id: 29_650_514,
    source_type: SourceType::Wikidata,
    name: "packPNM",
    extensions: &["ppn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
