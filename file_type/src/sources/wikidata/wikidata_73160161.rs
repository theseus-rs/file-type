use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73160161: FileFormat = FileFormat {
    id: 73_160_161,
    puid: "wikidata/73160161",
    name: "Visio Drawing Template",
    extensions: &["vst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
