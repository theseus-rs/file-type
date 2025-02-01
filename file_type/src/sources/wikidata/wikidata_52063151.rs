use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52063151: FileFormat = FileFormat {
    id: 52_063_151,
    puid: "wikidata/52063151",
    name: "Lotus Notes File",
    extensions: &["box"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
