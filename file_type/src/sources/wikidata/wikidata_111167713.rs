use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111167713: FileFormat = FileFormat {
    id: 111_167_713,
    source_type: SourceType::Wikidata,
    name: "ACD/CNMR Calculated Spectrum file",
    extensions: &["csp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
