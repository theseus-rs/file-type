use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344485: FileFormat = FileFormat {
    id: 28_344_485,
    puid: "wikidata/28344485",
    name: "HKI",
    extensions: &["hki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
