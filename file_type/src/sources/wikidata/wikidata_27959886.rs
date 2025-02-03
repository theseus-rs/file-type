use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959886: FileFormat = FileFormat {
    id: 27_959_886,
    source_type: SourceType::Wikidata,
    name: "Cubase song",
    extensions: &["all"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
