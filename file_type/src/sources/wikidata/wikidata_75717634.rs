use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75717634: FileFormat = FileFormat {
    id: 75_717_634,
    source_type: SourceType::Wikidata,
    name: "Corel Photo Paint User Defined Filter",
    extensions: &["usr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
