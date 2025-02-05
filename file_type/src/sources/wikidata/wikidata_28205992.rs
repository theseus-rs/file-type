use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205992: FileFormat = FileFormat {
    id: 28_205_992,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Alpha Channel",
    extensions: &["ima"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
