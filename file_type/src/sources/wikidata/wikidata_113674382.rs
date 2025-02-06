use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113674382: FileFormat = FileFormat {
    id: 113_674_382,
    source_type: SourceType::Wikidata,
    name: "File Converter Document",
    extensions: &["fcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
