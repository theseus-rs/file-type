use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127701093: FileFormat = FileFormat {
    id: 127_701_093,
    source_type: SourceType::Wikidata,
    name: "Hack source code file",
    extensions: &["hack"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
