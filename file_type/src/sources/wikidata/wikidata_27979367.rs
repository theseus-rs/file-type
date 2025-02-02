use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979367: FileFormat = FileFormat {
    id: 27_979_367,
    source_type: SourceType::Wikidata,
    name: "ReScene",
    extensions: &["srr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
