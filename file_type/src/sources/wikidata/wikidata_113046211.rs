use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113046211: FileFormat = FileFormat {
    id: 113_046_211,
    source_type: SourceType::Wikidata,
    name: "Live Code File Format",
    extensions: &["mlx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
