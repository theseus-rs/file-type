use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27491774: FileFormat = FileFormat {
    id: 27_491_774,
    puid: "wikidata/27491774",
    name: "7z, version 0.2 (with compression methods version 9.04)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
