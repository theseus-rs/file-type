use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131192273: FileFormat = FileFormat {
    id: 131_192_273,
    source_type: SourceType::Wikidata,
    name: "Tact source code file",
    extensions: &["tact"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
