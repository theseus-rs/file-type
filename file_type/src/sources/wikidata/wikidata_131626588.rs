use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131626588: FileFormat = FileFormat {
    id: 131_626_588,
    puid: "wikidata/131626588",
    name: "FLUENT CFF file format",
    extensions: &["dat.h5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
