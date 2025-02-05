use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55739293: FileFormat = FileFormat {
    id: 55_739_293,
    source_type: SourceType::Wikidata,
    name: "CRAM file format, version 1",
    extensions: &["cram"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
