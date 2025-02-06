use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975860: FileFormat = FileFormat {
    id: 28_975_860,
    source_type: SourceType::Wikidata,
    name: "OOGL MESH file",
    extensions: &["mesh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
