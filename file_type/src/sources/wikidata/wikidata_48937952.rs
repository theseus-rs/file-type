use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48937952: FileFormat = FileFormat {
    id: 48_937_952,
    source_type: SourceType::Wikidata,
    name: "descript.ion",
    extensions: &["ion"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
