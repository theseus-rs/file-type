use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_92442998: FileFormat = FileFormat {
    id: 92_442_998,
    puid: "wikidata/92442998",
    name: "Numpy compressed array format",
    extensions: &["npz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
