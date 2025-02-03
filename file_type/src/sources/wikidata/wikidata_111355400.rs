use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355400: FileFormat = FileFormat {
    id: 111_355_400,
    source_type: SourceType::Wikidata,
    name: "Annotated speech file",
    extensions: &["vap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
