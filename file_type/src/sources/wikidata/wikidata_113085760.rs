use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113085760: FileFormat = FileFormat {
    id: 113_085_760,
    source_type: SourceType::Wikidata,
    name: "CB7",
    extensions: &["cb7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
