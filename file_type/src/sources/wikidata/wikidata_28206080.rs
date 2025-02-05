use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206080: FileFormat = FileFormat {
    id: 28_206_080,
    source_type: SourceType::Wikidata,
    name: "PI6",
    extensions: &["PI6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
