use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51801210: FileFormat = FileFormat {
    id: 51_801_210,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Chart, version 29",
    extensions: &["xlc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
