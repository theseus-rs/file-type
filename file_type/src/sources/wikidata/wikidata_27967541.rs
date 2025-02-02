use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967541: FileFormat = FileFormat {
    id: 27_967_541,
    source_type: SourceType::Wikidata,
    name: "IFF-DEEP",
    extensions: &["deep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
