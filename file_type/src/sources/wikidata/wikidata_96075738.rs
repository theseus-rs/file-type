use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96075738: FileFormat = FileFormat {
    id: 96_075_738,
    source_type: SourceType::Wikidata,
    name: "Pajek format",
    extensions: &["net"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
