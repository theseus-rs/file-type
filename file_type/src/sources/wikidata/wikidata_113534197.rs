use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113534197: FileFormat = FileFormat {
    id: 113_534_197,
    source_type: SourceType::Wikidata,
    name: "Capture One Settings File",
    extensions: &["cos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
