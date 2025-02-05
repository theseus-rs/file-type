use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111341978: FileFormat = FileFormat {
    id: 111_341_978,
    source_type: SourceType::Wikidata,
    name: "MIDI Converter Studio packed Sound Font",
    extensions: &["sf2pack"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
