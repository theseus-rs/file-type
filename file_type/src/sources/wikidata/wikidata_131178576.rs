use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131178576: FileFormat = FileFormat {
    id: 131_178_576,
    source_type: SourceType::Wikidata,
    name: "SWIG source code file",
    extensions: &["swg"],
    media_types: &["text/swig"],
    signatures: &[],
    related_formats: &[],
};
