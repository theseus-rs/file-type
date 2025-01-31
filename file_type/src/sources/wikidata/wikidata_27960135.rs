use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960135: FileFormat = FileFormat {
    id: 27_960_135,
    puid: "wikidata/27960135",
    name: "INRS-Telecom file",
    extensions: &["aud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
