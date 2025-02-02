use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473308: FileFormat = FileFormat {
    id: 27_473_308,
    source_type: SourceType::Wikidata,
    name: "CADRG Frame File",
    extensions: &["ccz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
