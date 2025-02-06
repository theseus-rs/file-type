use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111182231: FileFormat = FileFormat {
    id: 111_182_231,
    source_type: SourceType::Wikidata,
    name: "ActionScript Communication File",
    extensions: &["asc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
