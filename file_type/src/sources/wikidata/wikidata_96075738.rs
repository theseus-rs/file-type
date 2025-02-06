use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96075738: FileFormat = FileFormat {
    id: 96_075_738,
    source_type: SourceType::Wikidata,
    name: "Pajek format",
    extensions: &["net"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
