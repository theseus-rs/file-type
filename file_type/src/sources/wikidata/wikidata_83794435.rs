use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83794435: FileFormat = FileFormat {
    id: 83_794_435,
    source_type: SourceType::Wikidata,
    name: "EclipseCrossword Word List File",
    extensions: &["ewl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
