use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650514: FileFormat = FileFormat {
    id: 29_650_514,
    source_type: SourceType::Wikidata,
    name: "packPNM",
    extensions: &["ppn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
