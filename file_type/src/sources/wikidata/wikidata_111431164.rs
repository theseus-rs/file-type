use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111431164: FileFormat = FileFormat {
    id: 111_431_164,
    source_type: SourceType::Wikidata,
    name: "C source code file",
    extensions: &["c"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
