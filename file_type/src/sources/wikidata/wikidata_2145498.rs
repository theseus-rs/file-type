use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2145498: FileFormat = FileFormat {
    id: 2_145_498,
    source_type: SourceType::Wikidata,
    name: "Requirements Interchange Format",
    extensions: &["reqif", "reqifz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
