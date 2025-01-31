use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27480195: FileFormat = FileFormat {
    id: 27_480_195,
    puid: "wikidata/27480195",
    name: "7z, version 0.2 (with compression methods version 4.56)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
