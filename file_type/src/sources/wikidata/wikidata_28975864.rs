use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975864: FileFormat = FileFormat {
    id: 28_975_864,
    source_type: SourceType::Wikidata,
    name: "OOGL Object File Format",
    extensions: &["off"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
