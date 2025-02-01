use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117423071: FileFormat = FileFormat {
    id: 117_423_071,
    puid: "wikidata/117423071",
    name: "Stimulus file",
    extensions: &["stm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
