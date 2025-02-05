use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62664835: FileFormat = FileFormat {
    id: 62_664_835,
    source_type: SourceType::Wikidata,
    name: "Active Server Page",
    extensions: &["asp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
