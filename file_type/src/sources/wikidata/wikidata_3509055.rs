use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3509055: FileFormat = FileFormat {
    id: 3_509_055,
    source_type: SourceType::Wikidata,
    name: "STEP file",
    extensions: &["p21", "step", "stp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
