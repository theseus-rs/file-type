use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129414825: FileFormat = FileFormat {
    id: 129_414_825,
    source_type: SourceType::Wikidata,
    name: "Golo source code file",
    extensions: &["golo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
