use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418328: FileFormat = FileFormat {
    id: 111_418_328,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Data File",
    extensions: &["abdata"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
