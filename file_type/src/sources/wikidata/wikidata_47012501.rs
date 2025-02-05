use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47012501: FileFormat = FileFormat {
    id: 47_012_501,
    source_type: SourceType::Wikidata,
    name: "OmniPage Pro Document file format",
    extensions: &["met", "opd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
