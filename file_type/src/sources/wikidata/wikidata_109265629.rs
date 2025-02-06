use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109265629: FileFormat = FileFormat {
    id: 109_265_629,
    source_type: SourceType::Wikidata,
    name: "MultiMate Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
