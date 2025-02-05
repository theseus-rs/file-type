use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1269709: FileFormat = FileFormat {
    id: 1_269_709,
    source_type: SourceType::Wikidata,
    name: "PHP Archive",
    extensions: &["phar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
