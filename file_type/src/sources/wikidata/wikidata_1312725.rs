use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1312725: FileFormat = FileFormat {
    id: 1_312_725,
    source_type: SourceType::Wikidata,
    name: "local shared object",
    extensions: &["sol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
