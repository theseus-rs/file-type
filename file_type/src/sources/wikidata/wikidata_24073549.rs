use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_24073549: FileFormat = FileFormat {
    id: 24_073_549,
    source_type: SourceType::Wikidata,
    name: "SFZ",
    extensions: &["sfz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
