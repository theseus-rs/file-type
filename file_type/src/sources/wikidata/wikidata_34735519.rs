use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34735519: FileFormat = FileFormat {
    id: 34_735_519,
    source_type: SourceType::Wikidata,
    name: "Signum font",
    extensions: &["e24", "l30", "p24", "p9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
