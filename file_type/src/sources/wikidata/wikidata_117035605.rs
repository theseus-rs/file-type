use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117035605: FileFormat = FileFormat {
    id: 117_035_605,
    source_type: SourceType::Wikidata,
    name: "VRML geography data",
    extensions: &["geo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
