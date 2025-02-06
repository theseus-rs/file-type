use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27349804: FileFormat = FileFormat {
    id: 27_349_804,
    source_type: SourceType::Wikidata,
    name: "ESRI Arc/Info Binary Grid",
    extensions: &["adf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
