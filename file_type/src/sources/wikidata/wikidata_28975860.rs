use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975860: FileFormat = FileFormat {
    id: 28_975_860,
    source_type: SourceType::Wikidata,
    name: "OOGL MESH file",
    extensions: &["mesh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
