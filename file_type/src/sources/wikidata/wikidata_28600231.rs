use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600231: FileFormat = FileFormat {
    id: 28_600_231,
    puid: "wikidata/28600231",
    name: "APW",
    extensions: &["apw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
