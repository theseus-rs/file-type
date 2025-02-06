use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116884421: FileFormat = FileFormat {
    id: 116_884_421,
    source_type: SourceType::Wikidata,
    name: "Adobe PhotoDeluxe data",
    extensions: &["pbd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
