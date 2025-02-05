use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205583: FileFormat = FileFormat {
    id: 28_205_583,
    source_type: SourceType::Wikidata,
    name: "OS/2 Pointer",
    extensions: &["ptr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
