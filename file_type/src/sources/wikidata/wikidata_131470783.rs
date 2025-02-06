use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131470783: FileFormat = FileFormat {
    id: 131_470_783,
    source_type: SourceType::Wikidata,
    name: "MetaImage Source file",
    extensions: &["mha"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
