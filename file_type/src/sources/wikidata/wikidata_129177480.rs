use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129177480: FileFormat = FileFormat {
    id: 129_177_480,
    source_type: SourceType::Wikidata,
    name: "Fennel source code file",
    extensions: &["fnl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
