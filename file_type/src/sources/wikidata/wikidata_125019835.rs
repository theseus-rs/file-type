use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125019835: FileFormat = FileFormat {
    id: 125_019_835,
    puid: "wikidata/125019835",
    name: "Sysex dump",
    extensions: &["syx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
