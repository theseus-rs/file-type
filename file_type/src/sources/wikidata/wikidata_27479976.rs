use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27479976: FileFormat = FileFormat {
    id: 27_479_976,
    puid: "wikidata/27479976",
    name: "7z, version 0.2 (with compression methods version 4.43)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
