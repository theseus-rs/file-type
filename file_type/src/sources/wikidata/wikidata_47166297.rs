use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47166297: FileFormat = FileFormat {
    id: 47_166_297,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Word Processor file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
