use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27479444: FileFormat = FileFormat {
    id: 27_479_444,
    puid: "wikidata/27479444",
    name: "7z, version 0.2 (with compression methods version 4.38)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
