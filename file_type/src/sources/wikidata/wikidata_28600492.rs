use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600492: FileFormat = FileFormat {
    id: 28_600_492,
    source_type: SourceType::Wikidata,
    name: "Data Resource File",
    extensions: &["drs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
