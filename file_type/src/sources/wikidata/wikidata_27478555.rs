use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27478555: FileFormat = FileFormat {
    id: 27_478_555,
    puid: "wikidata/27478555",
    name:
        "7z, version 0.2 (with compression methods version 4.16 beta, distributed with 7zip v4.20)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
