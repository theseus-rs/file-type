use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5010817: FileFormat = FileFormat {
    id: 5_010_817,
    source_type: SourceType::Wikidata,
    name: "CFS",
    extensions: &["cfs"],
    media_types: &["application/x-cfs-compressed"],
    signatures: &[],
    related_formats: &[],
};
