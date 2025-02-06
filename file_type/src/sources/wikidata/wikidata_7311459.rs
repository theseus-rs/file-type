use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7311459: FileFormat = FileFormat {
    id: 7_311_459,
    source_type: SourceType::Wikidata,
    name: "Relocatable Object Module Format",
    extensions: &["obj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
