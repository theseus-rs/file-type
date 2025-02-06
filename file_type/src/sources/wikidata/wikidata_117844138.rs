use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117844138: FileFormat = FileFormat {
    id: 117_844_138,
    source_type: SourceType::Wikidata,
    name: "Hayes JTFax file",
    extensions: &["jtf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
