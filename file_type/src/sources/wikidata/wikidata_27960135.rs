use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960135: FileFormat = FileFormat {
    id: 27_960_135,
    source_type: SourceType::Wikidata,
    name: "INRS-Telecom file",
    extensions: &["aud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
