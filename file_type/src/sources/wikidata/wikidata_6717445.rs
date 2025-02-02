use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6717445: FileFormat = FileFormat {
    id: 6_717_445,
    source_type: SourceType::Wikidata,
    name: "MRC",
    extensions: &["mrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
