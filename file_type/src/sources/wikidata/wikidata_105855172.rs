use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855172: FileFormat = FileFormat {
    id: 105_855_172,
    source_type: SourceType::Wikidata,
    name: "File-Type Image",
    extensions: &["fti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
