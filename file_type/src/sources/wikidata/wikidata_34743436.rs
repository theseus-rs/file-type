use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34743436: FileFormat = FileFormat {
    id: 34_743_436,
    source_type: SourceType::Wikidata,
    name: "Softlib",
    extensions: &["slb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
