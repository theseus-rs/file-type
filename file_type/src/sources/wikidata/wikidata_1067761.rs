use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1067761: FileFormat = FileFormat {
    id: 1_067_761,
    puid: "wikidata/1067761",
    name: "Windows Media Audio 9 Lossless",
    extensions: &["wma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
