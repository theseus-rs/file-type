use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113543215: FileFormat = FileFormat {
    id: 113_543_215,
    source_type: SourceType::Wikidata,
    name: "dBASE Windows Form File",
    extensions: &["wfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
