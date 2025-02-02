use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86450854: FileFormat = FileFormat {
    id: 86_450_854,
    source_type: SourceType::Wikidata,
    name: "ASICS",
    extensions: &["asics"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
