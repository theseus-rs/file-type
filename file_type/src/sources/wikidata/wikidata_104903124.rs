use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104903124: FileFormat = FileFormat {
    id: 104_903_124,
    source_type: SourceType::Wikidata,
    name: "Web Archive Collection Zipped",
    extensions: &["wacz"],
    media_types: &["application/x-wacz"],
    signatures: &[],
    related_formats: &[],
};
