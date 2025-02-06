use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131438617: FileFormat = FileFormat {
    id: 131_438_617,
    source_type: SourceType::Wikidata,
    name: "Xtend file format",
    extensions: &["xtend"],
    media_types: &["text/x-xtend"],
    signatures: &[],
    related_formats: &[],
};
