use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111316260: FileFormat = FileFormat {
    id: 111_316_260,
    source_type: SourceType::Wikidata,
    name: "Sample Cell II Mac instrument",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
