use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_92442998: FileFormat = FileFormat {
    id: 92_442_998,
    source_type: SourceType::Wikidata,
    name: "Numpy compressed array format",
    extensions: &["npz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
