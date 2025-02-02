use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125704723: FileFormat = FileFormat {
    id: 125_704_723,
    source_type: SourceType::Wikidata,
    name: "OpenOffice.org 1.0 Master Document",
    extensions: &["sxg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
