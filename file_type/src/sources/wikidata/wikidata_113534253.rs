use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113534253: FileFormat = FileFormat {
    id: 113_534_253,
    source_type: SourceType::Wikidata,
    name: "Geosoft Map Description File",
    extensions: &["mdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
