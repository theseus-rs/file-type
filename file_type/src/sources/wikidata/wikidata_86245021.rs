use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86245021: FileFormat = FileFormat {
    id: 86_245_021,
    source_type: SourceType::Wikidata,
    name: "BDOC 2",
    extensions: &["asice", "bdoc"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    signatures: &[],
    related_formats: &[],
};
