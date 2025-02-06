use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110501140: FileFormat = FileFormat {
    id: 110_501_140,
    source_type: SourceType::Wikidata,
    name: "Associated Signature Container Extended",
    extensions: &["asice", "sce"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    signatures: &[],
    related_formats: &[],
};
