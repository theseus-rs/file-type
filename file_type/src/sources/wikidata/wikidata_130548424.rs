use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130548424: FileFormat = FileFormat {
    id: 130_548_424,
    source_type: SourceType::Wikidata,
    name: "QBasic source code file",
    extensions: &["bas"],
    media_types: &["text/basic"],
    signatures: &[],
    related_formats: &[],
};
