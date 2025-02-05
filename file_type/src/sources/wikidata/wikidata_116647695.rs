use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116647695: FileFormat = FileFormat {
    id: 116_647_695,
    source_type: SourceType::Wikidata,
    name: "KeyForm 4.0 Document",
    extensions: &["kfm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
