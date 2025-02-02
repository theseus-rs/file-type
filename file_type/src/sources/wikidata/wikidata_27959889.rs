use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959889: FileFormat = FileFormat {
    id: 27_959_889,
    source_type: SourceType::Wikidata,
    name: "Cubase arrangement",
    extensions: &["arr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
