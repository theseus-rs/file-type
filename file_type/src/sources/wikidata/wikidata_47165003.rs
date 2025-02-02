use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47165003: FileFormat = FileFormat {
    id: 47_165_003,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project file format, version 4",
    extensions: &["mpp"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
