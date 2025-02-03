use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856625: FileFormat = FileFormat {
    id: 105_856_625,
    source_type: SourceType::Wikidata,
    name: "AIM Extended Wavefunction (with rem)",
    extensions: &["wfx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
