use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771302: FileFormat = FileFormat {
    id: 28_771_302,
    puid: "wikidata/28771302",
    name: "Matlab figure",
    extensions: &["fig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
