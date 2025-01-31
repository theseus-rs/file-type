use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967126: FileFormat = FileFormat {
    id: 27_967_126,
    puid: "wikidata/27967126",
    name: "CMR",
    extensions: &["cmr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
