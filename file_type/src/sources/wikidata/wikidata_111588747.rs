use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111588747: FileFormat = FileFormat {
    id: 111_588_747,
    source_type: SourceType::Wikidata,
    name: "Inspiration Software File",
    extensions: &["isf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
