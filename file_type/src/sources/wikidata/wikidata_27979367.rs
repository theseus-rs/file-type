use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979367: FileFormat = FileFormat {
    id: 27_979_367,
    source_type: SourceType::Wikidata,
    name: "ReScene",
    extensions: &["srr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
