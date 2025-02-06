use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1250383: FileFormat = FileFormat {
    id: 1_250_383,
    source_type: SourceType::Wikidata,
    name: "XYZ file format",
    extensions: &["xyz"],
    media_types: &["chemical/x-xyz"],
    signatures: &[],
    related_formats: &[],
};
