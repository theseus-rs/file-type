use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27479005: FileFormat = FileFormat {
    id: 27_479_005,
    puid: "wikidata/27479005",
    name: "7z, version 0.2 (with compression methods version 4.27, distributed with 7zip v4.28)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
