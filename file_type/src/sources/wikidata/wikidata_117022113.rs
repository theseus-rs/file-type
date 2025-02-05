use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117022113: FileFormat = FileFormat {
    id: 117_022_113,
    source_type: SourceType::Wikidata,
    name: "Garden File",
    extensions: &["grd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
