use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129571499: FileFormat = FileFormat {
    id: 129_571_499,
    puid: "wikidata/129571499",
    name: "HSAIL assembly code file",
    extensions: &["hsail"],
    media_types: &["text/x-hsail"],
    internal_signatures: &[],
    related_formats: &[],
};
