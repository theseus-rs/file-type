use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113557107: FileFormat = FileFormat {
    id: 113_557_107,
    source_type: SourceType::Wikidata,
    name: "Virtual CD-ROM, Uncompressed",
    extensions: &["fcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
