use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205763: FileFormat = FileFormat {
    id: 28_205_763,
    puid: "wikidata/28205763",
    name: "Binary Information File",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
