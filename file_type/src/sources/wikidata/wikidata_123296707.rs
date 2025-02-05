use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123296707: FileFormat = FileFormat {
    id: 123_296_707,
    source_type: SourceType::Wikidata,
    name: "CD-Face Layout",
    extensions: &["ntp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
