use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2713137: FileFormat = FileFormat {
    id: 2_713_137,
    source_type: SourceType::Wikidata,
    name: "Crystallographic Information File",
    extensions: &["cif"],
    media_types: &["chemical/x-cif"],
    internal_signatures: &[],
    related_formats: &[],
};
