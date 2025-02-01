use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27492954: FileFormat = FileFormat {
    id: 27_492_954,
    puid: "wikidata/27492954",
    name: "7z, version 0.2 (with compression methods version 16.03)",
    extensions: &["7z"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
