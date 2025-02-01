use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33515758: FileFormat = FileFormat {
    id: 33_515_758,
    puid: "wikidata/33515758",
    name: "LAS 1.4 file format",
    extensions: &["las"],
    media_types: &["application/vnd.las"],
    internal_signatures: &[],
    related_formats: &[],
};
