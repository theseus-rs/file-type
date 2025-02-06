use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34743436: FileFormat = FileFormat {
    id: 34_743_436,
    source_type: SourceType::Wikidata,
    name: "Softlib",
    extensions: &["slb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
