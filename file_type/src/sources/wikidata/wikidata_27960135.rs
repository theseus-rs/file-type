use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960135: FileFormat = FileFormat {
    id: 27_960_135,
    source_type: SourceType::Wikidata,
    name: "INRS-Telecom file",
    extensions: &["aud"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
