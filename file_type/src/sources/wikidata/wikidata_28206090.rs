use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206090: FileFormat = FileFormat {
    id: 28_206_090,
    puid: "wikidata/28206090",
    name: "TT Low Resolution",
    extensions: &["PI8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
