use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851526: FileFormat = FileFormat {
    id: 105_851_526,
    source_type: SourceType::Wikidata,
    name: "Camtasia Project",
    extensions: &["tscproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
