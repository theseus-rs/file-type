use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125019835: FileFormat = FileFormat {
    id: 125_019_835,
    source_type: SourceType::Wikidata,
    name: "Sysex dump",
    extensions: &["syx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
