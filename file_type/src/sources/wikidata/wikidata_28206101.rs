use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206101: FileFormat = FileFormat {
    id: 28_206_101,
    source_type: SourceType::Wikidata,
    name: "FaceSaver",
    extensions: &["fac", "face"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
