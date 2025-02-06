use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1192568: FileFormat = FileFormat {
    id: 1_192_568,
    source_type: SourceType::Wikidata,
    name: ".sys",
    extensions: &["sys"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
