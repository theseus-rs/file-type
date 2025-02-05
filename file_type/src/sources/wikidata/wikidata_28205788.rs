use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205788: FileFormat = FileFormat {
    id: 28_205_788,
    source_type: SourceType::Wikidata,
    name: "Compact Picture Format",
    extensions: &["cpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
