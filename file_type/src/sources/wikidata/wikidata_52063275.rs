use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063275: FileFormat = FileFormat {
    id: 52_063_275,
    source_type: SourceType::Wikidata,
    name: "Professional Write Text File",
    extensions: &["pw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
