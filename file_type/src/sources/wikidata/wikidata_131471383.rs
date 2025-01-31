use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131471383: FileFormat = FileFormat {
    id: 131_471_383,
    puid: "wikidata/131471383",
    name: "Compressed MGH file format",
    extensions: &["mgh.gz", "mgz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
