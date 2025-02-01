use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_70081522: FileFormat = FileFormat {
    id: 70_081_522,
    puid: "wikidata/70081522",
    name: "TextPipe Filter List file format",
    extensions: &["fll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
