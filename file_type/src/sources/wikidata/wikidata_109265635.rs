use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109265635: FileFormat = FileFormat {
    id: 109_265_635,
    source_type: SourceType::Wikidata,
    name: "Pro Write Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
