use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66141873: FileFormat = FileFormat {
    id: 66_141_873,
    source_type: SourceType::Wikidata,
    name: "MDE file format",
    extensions: &["mde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
