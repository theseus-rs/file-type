use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124843932: FileFormat = FileFormat {
    id: 124_843_932,
    source_type: SourceType::Wikidata,
    name: "Apple Contacts Archive file",
    extensions: &["abbu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
