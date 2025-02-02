use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125041640: FileFormat = FileFormat {
    id: 125_041_640,
    source_type: SourceType::Wikidata,
    name: "Yoshimi Instrument File",
    extensions: &["xiy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
