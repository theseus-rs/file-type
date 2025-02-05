use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28447338: FileFormat = FileFormat {
    id: 28_447_338,
    source_type: SourceType::Wikidata,
    name: "Digital Document",
    extensions: &["ddoc"],
    media_types: &["application/ddoc"],
    signatures: &[],
    related_formats: &[],
};
