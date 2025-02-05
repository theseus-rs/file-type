use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205626: FileFormat = FileFormat {
    id: 28_205_626,
    source_type: SourceType::Wikidata,
    name: "Sun icon",
    extensions: &["ico", "icon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
