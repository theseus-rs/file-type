use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27478587: FileFormat = FileFormat {
    id: 27_478_587,
    puid: "wikidata/27478587",
    name:
        "7z, version 0.2 (with compression methods version 4.16 beta, distributed with 7zip v4.26)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
