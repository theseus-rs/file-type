use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28346237: FileFormat = FileFormat {
    id: 28_346_237,
    source_type: SourceType::Wikidata,
    name: "TTDDD",
    extensions: &["ttd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
