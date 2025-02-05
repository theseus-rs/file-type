use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967541: FileFormat = FileFormat {
    id: 27_967_541,
    source_type: SourceType::Wikidata,
    name: "IFF-DEEP",
    extensions: &["deep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
