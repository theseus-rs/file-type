use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855172: FileFormat = FileFormat {
    id: 105_855_172,
    source_type: SourceType::Wikidata,
    name: "File-Type Image",
    extensions: &["fti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
