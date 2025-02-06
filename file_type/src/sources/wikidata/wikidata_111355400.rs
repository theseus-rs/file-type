use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355400: FileFormat = FileFormat {
    id: 111_355_400,
    source_type: SourceType::Wikidata,
    name: "Annotated speech file",
    extensions: &["vap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
