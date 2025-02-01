use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108836986: FileFormat = FileFormat {
    id: 108_836_986,
    puid: "wikidata/108836986",
    name: "Nero Audio Compilation",
    extensions: &["nra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
