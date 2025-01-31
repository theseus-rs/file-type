use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27480264: FileFormat = FileFormat {
    id: 27_480_264,
    puid: "wikidata/27480264",
    name: "7z, version 0.2 (with compression methods version 4.65)",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
