use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47166177: FileFormat = FileFormat {
    id: 47_166_177,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Drawing file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
