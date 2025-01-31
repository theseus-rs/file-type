use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_524090: FileFormat = FileFormat {
    id: 524_090,
    puid: "wikidata/524090",
    name: "MT9",
    extensions: &["mt9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
