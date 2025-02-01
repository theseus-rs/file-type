use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48805099: FileFormat = FileFormat {
    id: 48_805_099,
    puid: "wikidata/48805099",
    name: "Btrieve Database",
    extensions: &["btr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
