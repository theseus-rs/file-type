use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131748260: FileFormat = FileFormat {
    id: 131_748_260,
    source_type: SourceType::Wikidata,
    name: "Parallel Input Output file",
    extensions: &["pio"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
