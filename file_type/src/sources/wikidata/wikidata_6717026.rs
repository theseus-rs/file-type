use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6717026: FileFormat = FileFormat {
    id: 6_717_026,
    source_type: SourceType::Wikidata,
    name: "MOI",
    extensions: &["moi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
