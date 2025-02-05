use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205805: FileFormat = FileFormat {
    id: 28_205_805,
    source_type: SourceType::Wikidata,
    name: "Object File Format, binary variant",
    extensions: &["off"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
