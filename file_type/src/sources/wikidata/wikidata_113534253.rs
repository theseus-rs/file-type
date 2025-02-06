use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113534253: FileFormat = FileFormat {
    id: 113_534_253,
    source_type: SourceType::Wikidata,
    name: "Geosoft Map Description File",
    extensions: &["mdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
