use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2713137: FileFormat = FileFormat {
    id: 2_713_137,
    source_type: SourceType::Wikidata,
    name: "Crystallographic Information File",
    extensions: &["cif"],
    media_types: &["chemical/x-cif"],
    signatures: &[],
    related_formats: &[],
};
