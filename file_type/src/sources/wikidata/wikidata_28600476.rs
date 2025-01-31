use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600476: FileFormat = FileFormat {
    id: 28_600_476,
    puid: "wikidata/28600476",
    name: "DOS device driver",
    extensions: &["dos", "sys"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
