use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207447: FileFormat = FileFormat {
    id: 28_207_447,
    source_type: SourceType::Wikidata,
    name: "VIPS",
    extensions: &["v"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
