use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132155960: FileFormat = FileFormat {
    id: 132_155_960,
    source_type: SourceType::Wikidata,
    name: "Braille text file format",
    extensions: &["brl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
