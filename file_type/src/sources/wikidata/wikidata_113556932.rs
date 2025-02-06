use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113556932: FileFormat = FileFormat {
    id: 113_556_932,
    source_type: SourceType::Wikidata,
    name: "Duplicator Image format",
    extensions: &["dao"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
