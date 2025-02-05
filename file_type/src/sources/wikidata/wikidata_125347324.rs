use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125347324: FileFormat = FileFormat {
    id: 125_347_324,
    source_type: SourceType::Wikidata,
    name: "Binary Grid File",
    extensions: &["grb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
