use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116851698: FileFormat = FileFormat {
    id: 116_851_698,
    puid: "wikidata/116851698",
    name: "VersaCheck Data File",
    extensions: &["vdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
