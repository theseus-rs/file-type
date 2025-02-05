use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117035605: FileFormat = FileFormat {
    id: 117_035_605,
    source_type: SourceType::Wikidata,
    name: "VRML geography data",
    extensions: &["geo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
