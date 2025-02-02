use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975638: FileFormat = FileFormat {
    id: 28_975_638,
    source_type: SourceType::Wikidata,
    name: "Parasolid",
    extensions: &["x_t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
