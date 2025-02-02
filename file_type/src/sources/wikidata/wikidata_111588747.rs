use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111588747: FileFormat = FileFormat {
    id: 111_588_747,
    source_type: SourceType::Wikidata,
    name: "Inspiration Software File",
    extensions: &["isf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
