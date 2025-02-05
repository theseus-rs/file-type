use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116418918: FileFormat = FileFormat {
    id: 116_418_918,
    source_type: SourceType::Wikidata,
    name: "Adobe Photoshop Color Table",
    extensions: &["act"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
