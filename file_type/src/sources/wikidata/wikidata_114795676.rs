use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114795676: FileFormat = FileFormat {
    id: 114_795_676,
    source_type: SourceType::Wikidata,
    name: "ReadCube Enhanced PDF",
    extensions: &["epdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
