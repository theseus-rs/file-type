use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47529212: FileFormat = FileFormat {
    id: 47_529_212,
    source_type: SourceType::Wikidata,
    name: "VLW font file",
    extensions: &["vlw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
