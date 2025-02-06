use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51913144: FileFormat = FileFormat {
    id: 51_913_144,
    source_type: SourceType::Wikidata,
    name: "Instalit Script",
    extensions: &["pvd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
