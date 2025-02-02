use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1269709: FileFormat = FileFormat {
    id: 1_269_709,
    source_type: SourceType::Wikidata,
    name: "PHP Archive",
    extensions: &["phar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
