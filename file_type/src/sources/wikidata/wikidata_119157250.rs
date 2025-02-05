use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119157250: FileFormat = FileFormat {
    id: 119_157_250,
    source_type: SourceType::Wikidata,
    name: "Digital Image Publishing File",
    extensions: &["php"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
