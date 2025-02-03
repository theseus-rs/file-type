use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60000066: FileFormat = FileFormat {
    id: 60_000_066,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Owner File",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
