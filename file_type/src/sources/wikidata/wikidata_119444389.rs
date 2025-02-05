use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119444389: FileFormat = FileFormat {
    id: 119_444_389,
    source_type: SourceType::Wikidata,
    name: "Comic Book Template",
    extensions: &["cbtx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
