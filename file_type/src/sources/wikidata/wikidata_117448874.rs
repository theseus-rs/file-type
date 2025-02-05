use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448874: FileFormat = FileFormat {
    id: 117_448_874,
    source_type: SourceType::Wikidata,
    name: "Transcriber TRS Format",
    extensions: &["trs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
