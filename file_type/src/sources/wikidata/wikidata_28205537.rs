use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205537: FileFormat = FileFormat {
    id: 28_205_537,
    source_type: SourceType::Wikidata,
    name: "Micrografx Icon",
    extensions: &["icn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
