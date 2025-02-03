use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60371443: FileFormat = FileFormat {
    id: 60_371_443,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Report File",
    extensions: &["qxp_report"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
