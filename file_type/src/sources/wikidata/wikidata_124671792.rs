use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124671792: FileFormat = FileFormat {
    id: 124_671_792,
    source_type: SourceType::Wikidata,
    name: "Archive eXchange Format",
    extensions: &["axf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
