use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116520564: FileFormat = FileFormat {
    id: 116_520_564,
    source_type: SourceType::Wikidata,
    name: "PHP 3 Web Page",
    extensions: &["php3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
