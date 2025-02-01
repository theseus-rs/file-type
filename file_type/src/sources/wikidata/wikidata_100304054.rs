use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100304054: FileFormat = FileFormat {
    id: 100_304_054,
    puid: "wikidata/100304054",
    name: "Flow Charting Graphic Flowcharting Image",
    extensions: &["gfi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
