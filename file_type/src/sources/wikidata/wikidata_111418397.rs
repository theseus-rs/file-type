use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111418397: FileFormat = FileFormat {
    id: 111_418_397,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge Cache Export File",
    extensions: &["bridgecache"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
