use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693790: FileFormat = FileFormat {
    id: 128_693_790,
    source_type: SourceType::Wikidata,
    name: "Boa file format",
    extensions: &["boa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
