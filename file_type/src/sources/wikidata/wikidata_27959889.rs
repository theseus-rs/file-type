use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959889: FileFormat = FileFormat {
    id: 27_959_889,
    source_type: SourceType::Wikidata,
    name: "Cubase arrangement",
    extensions: &["arr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
