use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129177252: FileFormat = FileFormat {
    id: 129_177_252,
    source_type: SourceType::Wikidata,
    name: "Felix source code file",
    extensions: &["flx"],
    media_types: &["text/x-felix"],
    internal_signatures: &[],
    related_formats: &[],
};
