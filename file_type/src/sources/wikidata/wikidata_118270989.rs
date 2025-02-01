use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118270989: FileFormat = FileFormat {
    id: 118_270_989,
    puid: "wikidata/118270989",
    name: "Altamira Composer file",
    extensions: &["acc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
