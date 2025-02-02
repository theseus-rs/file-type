use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99972520: FileFormat = FileFormat {
    id: 99_972_520,
    source_type: SourceType::Wikidata,
    name: "OmniPage Pro Document 10",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
