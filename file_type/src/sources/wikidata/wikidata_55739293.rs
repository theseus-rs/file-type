use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55739293: FileFormat = FileFormat {
    id: 55_739_293,
    source_type: SourceType::Wikidata,
    name: "CRAM file format, version 1",
    extensions: &["cram"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
