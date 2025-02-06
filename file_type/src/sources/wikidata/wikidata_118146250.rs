use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118146250: FileFormat = FileFormat {
    id: 118_146_250,
    source_type: SourceType::Wikidata,
    name: "Stripline File",
    extensions: &["tl4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
