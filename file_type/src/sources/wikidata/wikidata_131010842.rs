use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131010842: FileFormat = FileFormat {
    id: 131_010_842,
    source_type: SourceType::Wikidata,
    name: "Smithy file format",
    extensions: &["smithy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
