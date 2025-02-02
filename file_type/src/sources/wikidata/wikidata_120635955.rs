use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120635955: FileFormat = FileFormat {
    id: 120_635_955,
    source_type: SourceType::Wikidata,
    name: "Microsoft Data Access Page",
    extensions: &["htm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
