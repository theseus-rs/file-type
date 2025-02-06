use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125019835: FileFormat = FileFormat {
    id: 125_019_835,
    source_type: SourceType::Wikidata,
    name: "Sysex dump",
    extensions: &["syx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
