use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124969775: FileFormat = FileFormat {
    id: 124_969_775,
    source_type: SourceType::Wikidata,
    name: "Songsmith file",
    extensions: &["songsmith"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
