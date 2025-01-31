use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122730741: FileFormat = FileFormat {
    id: 122_730_741,
    puid: "wikidata/122730741",
    name: "Lazer View file",
    extensions: &["lv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
