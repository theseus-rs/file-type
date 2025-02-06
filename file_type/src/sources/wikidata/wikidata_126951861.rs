use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126951861: FileFormat = FileFormat {
    id: 126_951_861,
    source_type: SourceType::Wikidata,
    name: "Scala source code file",
    extensions: &["scala"],
    media_types: &["text/x-scala"],
    signatures: &[],
    related_formats: &[],
};
