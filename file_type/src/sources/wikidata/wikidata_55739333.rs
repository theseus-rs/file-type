use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55739333: FileFormat = FileFormat {
    id: 55_739_333,
    source_type: SourceType::Wikidata,
    name: "CRAM file format, version 2",
    extensions: &["cram"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
