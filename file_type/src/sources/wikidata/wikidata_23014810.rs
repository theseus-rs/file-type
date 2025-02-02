use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_23014810: FileFormat = FileFormat {
    id: 23_014_810,
    source_type: SourceType::Wikidata,
    name: "chr",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
