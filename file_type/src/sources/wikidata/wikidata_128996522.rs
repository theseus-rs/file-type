use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128996522: FileFormat = FileFormat {
    id: 128_996_522,
    source_type: SourceType::Wikidata,
    name: "Easytrieve file format",
    extensions: &["ezt"],
    media_types: &["text/x-easytrieve"],
    signatures: &[],
    related_formats: &[],
};
