use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355316: FileFormat = FileFormat {
    id: 111_355_316,
    source_type: SourceType::Wikidata,
    name: "UltraTracker wave file",
    extensions: &["uwf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
