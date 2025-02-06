use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856121: FileFormat = FileFormat {
    id: 105_856_121,
    source_type: SourceType::Wikidata,
    name: "Device Tree Source (with rem)",
    extensions: &["dts"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
