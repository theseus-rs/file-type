use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122412480: FileFormat = FileFormat {
    id: 122_412_480,
    source_type: SourceType::Wikidata,
    name: "Merge File",
    extensions: &["mer"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
