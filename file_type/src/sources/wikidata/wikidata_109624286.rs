use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109624286: FileFormat = FileFormat {
    id: 109_624_286,
    source_type: SourceType::Wikidata,
    name: "WebPlus Essentials Site",
    extensions: &["wpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
