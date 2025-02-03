use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967191: FileFormat = FileFormat {
    id: 27_967_191,
    source_type: SourceType::Wikidata,
    name: "Grave Composer module",
    extensions: &["wow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
