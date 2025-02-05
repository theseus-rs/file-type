use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111316574: FileFormat = FileFormat {
    id: 111_316_574,
    source_type: SourceType::Wikidata,
    name: "Sample Cell II PC instrument",
    extensions: &["ins"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
