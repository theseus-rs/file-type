use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60371443: FileFormat = FileFormat {
    id: 60_371_443,
    source_type: SourceType::Wikidata,
    name: "Quark Xpress Report File",
    extensions: &["qxp_report"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
