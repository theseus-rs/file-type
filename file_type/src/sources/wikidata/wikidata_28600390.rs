use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600390: FileFormat = FileFormat {
    id: 28_600_390,
    puid: "wikidata/28600390",
    name: "Apple framework",
    extensions: &["framework"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
