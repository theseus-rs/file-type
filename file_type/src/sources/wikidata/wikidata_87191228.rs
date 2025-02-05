use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87191228: FileFormat = FileFormat {
    id: 87_191_228,
    source_type: SourceType::Wikidata,
    name: "X3D 3.2",
    extensions: &["x3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
