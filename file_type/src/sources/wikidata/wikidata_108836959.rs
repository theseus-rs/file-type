use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108836959: FileFormat = FileFormat {
    id: 108_836_959,
    puid: "wikidata/108836959",
    name: "Nero ISO CD Compilation File",
    extensions: &["nri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
