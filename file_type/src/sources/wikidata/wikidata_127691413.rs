use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127691413: FileFormat = FileFormat {
    id: 127_691_413,
    source_type: SourceType::Wikidata,
    name: "Elm file",
    extensions: &["elm"],
    media_types: &["text/x-elm"],
    signatures: &[],
    related_formats: &[],
};
