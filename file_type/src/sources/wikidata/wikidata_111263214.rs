use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263214: FileFormat = FileFormat {
    id: 111_263_214,
    source_type: SourceType::Wikidata,
    name: "3GPP 'AMR interface format 2'",
    extensions: &["cod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
