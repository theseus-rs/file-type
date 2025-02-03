use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62619688: FileFormat = FileFormat {
    id: 62_619_688,
    source_type: SourceType::Wikidata,
    name: "7-bit ASCII Text",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
