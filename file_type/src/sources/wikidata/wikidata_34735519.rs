use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34735519: FileFormat = FileFormat {
    id: 34_735_519,
    source_type: SourceType::Wikidata,
    name: "Signum font",
    extensions: &["e24", "l30", "p24", "p9"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
