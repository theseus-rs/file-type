use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851526: FileFormat = FileFormat {
    id: 105_851_526,
    source_type: SourceType::Wikidata,
    name: "Camtasia Project",
    extensions: &["tscproj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
