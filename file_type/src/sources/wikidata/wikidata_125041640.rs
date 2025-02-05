use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125041640: FileFormat = FileFormat {
    id: 125_041_640,
    source_type: SourceType::Wikidata,
    name: "Yoshimi Instrument File",
    extensions: &["xiy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
