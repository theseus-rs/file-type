use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959810: FileFormat = FileFormat {
    id: 27_959_810,
    puid: "wikidata/27959810",
    name: "Ableton Live Set",
    extensions: &["als"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
