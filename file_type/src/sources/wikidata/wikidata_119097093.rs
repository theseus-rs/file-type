use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119097093: FileFormat = FileFormat {
    id: 119_097_093,
    source_type: SourceType::Wikidata,
    name: "Fireworks PNG",
    extensions: &["png"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
