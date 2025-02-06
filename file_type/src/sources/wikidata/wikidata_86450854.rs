use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86450854: FileFormat = FileFormat {
    id: 86_450_854,
    source_type: SourceType::Wikidata,
    name: "ASICS",
    extensions: &["asics"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    signatures: &[],
    related_formats: &[],
};
