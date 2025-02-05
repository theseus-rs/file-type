use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116145260: FileFormat = FileFormat {
    id: 116_145_260,
    source_type: SourceType::Wikidata,
    name: "FIT file",
    extensions: &["fit"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
