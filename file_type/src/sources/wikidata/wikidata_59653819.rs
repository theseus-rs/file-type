use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59653819: FileFormat = FileFormat {
    id: 59_653_819,
    source_type: SourceType::Wikidata,
    name: "Maya Binary File Format, 64 bit",
    extensions: &["mb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
