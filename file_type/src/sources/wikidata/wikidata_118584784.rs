use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118584784: FileFormat = FileFormat {
    id: 118_584_784,
    puid: "wikidata/118584784",
    name: "Cakewalk Bundle",
    extensions: &["cwb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
