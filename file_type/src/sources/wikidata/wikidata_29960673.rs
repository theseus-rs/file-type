use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29960673: FileFormat = FileFormat {
    id: 29_960_673,
    source_type: SourceType::Wikidata,
    name: "Avantes USB spectrometer ROH file",
    extensions: &["roh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
