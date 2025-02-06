use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2011664: FileFormat = FileFormat {
    id: 2_011_664,
    source_type: SourceType::Wikidata,
    name: "Object File Format",
    extensions: &["off"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
