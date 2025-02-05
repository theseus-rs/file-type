use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130711801: FileFormat = FileFormat {
    id: 130_711_801,
    source_type: SourceType::Wikidata,
    name: "RPMSpec file format",
    extensions: &["spec"],
    media_types: &["text/x-rpm-spec"],
    signatures: &[],
    related_formats: &[],
};
