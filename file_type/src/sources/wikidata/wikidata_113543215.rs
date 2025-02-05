use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113543215: FileFormat = FileFormat {
    id: 113_543_215,
    source_type: SourceType::Wikidata,
    name: "dBASE Windows Form File",
    extensions: &["wfm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
