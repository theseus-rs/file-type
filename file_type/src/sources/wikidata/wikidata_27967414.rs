use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967414: FileFormat = FileFormat {
    id: 27_967_414,
    source_type: SourceType::Wikidata,
    name: "Instrument Bank",
    extensions: &["ibk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
