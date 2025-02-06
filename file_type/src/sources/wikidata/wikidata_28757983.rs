use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757983: FileFormat = FileFormat {
    id: 28_757_983,
    source_type: SourceType::Wikidata,
    name: "IPF",
    extensions: &["ipf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
