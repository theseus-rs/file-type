use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75536482: FileFormat = FileFormat {
    id: 75_536_482,
    puid: "wikidata/75536482",
    name: "Ulead Photo Express image",
    extensions: &["upx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
