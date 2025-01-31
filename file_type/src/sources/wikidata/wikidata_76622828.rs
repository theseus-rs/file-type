use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76622828: FileFormat = FileFormat {
    id: 76_622_828,
    puid: "wikidata/76622828",
    name: "WOLF eBook",
    extensions: &["wol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
