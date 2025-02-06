use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966966: FileFormat = FileFormat {
    id: 27_966_966,
    source_type: SourceType::Wikidata,
    name: "Accolade MIDI File Format",
    extensions: &["mus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
