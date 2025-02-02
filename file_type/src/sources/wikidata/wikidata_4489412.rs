use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4489412: FileFormat = FileFormat {
    id: 4_489_412,
    source_type: SourceType::Wikidata,
    name: "ARFF",
    extensions: &["arff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
