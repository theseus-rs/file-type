use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206101: FileFormat = FileFormat {
    id: 28_206_101,
    source_type: SourceType::Wikidata,
    name: "FaceSaver",
    extensions: &["fac", "face"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
