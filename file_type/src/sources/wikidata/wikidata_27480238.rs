use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27480238: FileFormat = FileFormat {
    id: 27_480_238,
    puid: "wikidata/27480238",
    name: "7z, version 0.2 (with compression methods version 4.61)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
