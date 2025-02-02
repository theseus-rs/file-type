use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967414: FileFormat = FileFormat {
    id: 27_967_414,
    source_type: SourceType::Wikidata,
    name: "Instrument Bank",
    extensions: &["ibk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
