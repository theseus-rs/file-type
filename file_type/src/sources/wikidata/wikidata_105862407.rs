use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862407: FileFormat = FileFormat {
    id: 105_862_407,
    source_type: SourceType::Wikidata,
    name: "Mac Compact Pro archive",
    extensions: &["cpt"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
