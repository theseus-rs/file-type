use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115116796: FileFormat = FileFormat {
    id: 115_116_796,
    source_type: SourceType::Wikidata,
    name: "Gunpaint Image File",
    extensions: &["gun"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
