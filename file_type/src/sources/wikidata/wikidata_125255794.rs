use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125255794: FileFormat = FileFormat {
    id: 125_255_794,
    puid: "wikidata/125255794",
    name: "CombiTimeTable",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
