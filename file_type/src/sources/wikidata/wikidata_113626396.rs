use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113626396: FileFormat = FileFormat {
    id: 113_626_396,
    source_type: SourceType::Wikidata,
    name: "ScatterShow file",
    extensions: &["scattershow"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
