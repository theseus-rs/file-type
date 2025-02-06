use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116520564: FileFormat = FileFormat {
    id: 116_520_564,
    source_type: SourceType::Wikidata,
    name: "PHP 3 Web Page",
    extensions: &["php3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
