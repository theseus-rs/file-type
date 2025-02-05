use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87191251: FileFormat = FileFormat {
    id: 87_191_251,
    source_type: SourceType::Wikidata,
    name: "X3D 3.3",
    extensions: &["x3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
