use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128210388: FileFormat = FileFormat {
    id: 128_210_388,
    puid: "wikidata/128210388",
    name: "Xcode config",
    extensions: &["xcconfig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
