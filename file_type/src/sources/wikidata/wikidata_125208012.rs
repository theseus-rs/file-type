use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125208012: FileFormat = FileFormat {
    id: 125_208_012,
    source_type: SourceType::Wikidata,
    name: "TaskJuggler project",
    extensions: &["tjp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
