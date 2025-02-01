use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341978: FileFormat = FileFormat {
    id: 111_341_978,
    puid: "wikidata/111341978",
    name: "MIDI Converter Studio packed Sound Font",
    extensions: &["sf2pack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
