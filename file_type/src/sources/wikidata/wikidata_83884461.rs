use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83884461: FileFormat = FileFormat {
    id: 83_884_461,
    source_type: SourceType::Wikidata,
    name: "Windows Address Book",
    extensions: &["wab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
