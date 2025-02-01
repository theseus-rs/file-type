use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967182: FileFormat = FileFormat {
    id: 27_967_182,
    puid: "wikidata/27967182",
    name: "Farandole Composer sample",
    extensions: &["fsm", "usm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
