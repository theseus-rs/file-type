use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4545331: FileFormat = FileFormat {
    id: 4_545_331,
    puid: "wikidata/4545331",
    name: ".3ds",
    extensions: &["3ds", "3ds"],
    media_types: &["application/x-3ds", "image/x-3ds"],
    internal_signatures: &[],
    related_formats: &[],
};
