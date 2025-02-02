use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48623384: FileFormat = FileFormat {
    id: 48_623_384,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project Export File",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    internal_signatures: &[],
    related_formats: &[],
};
