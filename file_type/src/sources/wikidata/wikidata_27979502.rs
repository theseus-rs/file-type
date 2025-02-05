use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979502: FileFormat = FileFormat {
    id: 27_979_502,
    source_type: SourceType::Wikidata,
    name: "DNG camera profile",
    extensions: &["dcp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
