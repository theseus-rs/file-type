use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600260: FileFormat = FileFormat {
    id: 28_600_260,
    source_type: SourceType::Wikidata,
    name: "AWD",
    extensions: &["awd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
