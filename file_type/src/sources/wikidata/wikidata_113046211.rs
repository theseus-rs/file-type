use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113046211: FileFormat = FileFormat {
    id: 113_046_211,
    source_type: SourceType::Wikidata,
    name: "Live Code File Format",
    extensions: &["mlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
