use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128694654: FileFormat = FileFormat {
    id: 128_694_654,
    source_type: SourceType::Wikidata,
    name: "Carbon file format",
    extensions: &["carbon"],
    media_types: &["text/x-carbon"],
    signatures: &[],
    related_formats: &[],
};
