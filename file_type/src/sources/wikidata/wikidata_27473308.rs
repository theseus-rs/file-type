use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473308: FileFormat = FileFormat {
    id: 27_473_308,
    source_type: SourceType::Wikidata,
    name: "CADRG Frame File",
    extensions: &["ccz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
