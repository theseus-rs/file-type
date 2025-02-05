use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_92442998: FileFormat = FileFormat {
    id: 92_442_998,
    source_type: SourceType::Wikidata,
    name: "Numpy compressed array format",
    extensions: &["npz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
