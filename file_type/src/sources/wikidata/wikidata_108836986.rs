use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108836986: FileFormat = FileFormat {
    id: 108_836_986,
    source_type: SourceType::Wikidata,
    name: "Nero Audio Compilation",
    extensions: &["nra"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
