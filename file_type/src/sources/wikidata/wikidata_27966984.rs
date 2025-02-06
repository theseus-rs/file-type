use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966984: FileFormat = FileFormat {
    id: 27_966_984,
    source_type: SourceType::Wikidata,
    name: "Actionamics Sound Tool",
    extensions: &["ast"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
