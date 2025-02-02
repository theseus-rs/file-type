use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113301729: FileFormat = FileFormat {
    id: 113_301_729,
    source_type: SourceType::Wikidata,
    name: "Sonic Foundry Audio",
    extensions: &["sfa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
