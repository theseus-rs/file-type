use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47529212: FileFormat = FileFormat {
    id: 47_529_212,
    source_type: SourceType::Wikidata,
    name: "VLW font file",
    extensions: &["vlw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
