use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111167713: FileFormat = FileFormat {
    id: 111_167_713,
    source_type: SourceType::Wikidata,
    name: "ACD/CNMR Calculated Spectrum file",
    extensions: &["csp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
