use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122229335: FileFormat = FileFormat {
    id: 122_229_335,
    puid: "wikidata/122229335",
    name: "WPA-PSK Export Hash",
    extensions: &["wph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
