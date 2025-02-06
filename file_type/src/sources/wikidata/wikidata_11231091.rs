use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_11231091: FileFormat = FileFormat {
    id: 11_231_091,
    source_type: SourceType::Wikidata,
    name: "MASL",
    extensions: &["Msl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
