use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48623760: FileFormat = FileFormat {
    id: 48_623_760,
    source_type: SourceType::Wikidata,
    name: "Paint Shop Pro Image, version 5",
    extensions: &["psp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
