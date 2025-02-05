use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125704723: FileFormat = FileFormat {
    id: 125_704_723,
    source_type: SourceType::Wikidata,
    name: "OpenOffice.org 1.0 Master Document",
    extensions: &["sxg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
