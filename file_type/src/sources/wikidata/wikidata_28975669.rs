use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975669: FileFormat = FileFormat {
    id: 28_975_669,
    source_type: SourceType::Wikidata,
    name: "BMF",
    extensions: &["bmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
