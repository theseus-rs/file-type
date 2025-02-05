use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126822716: FileFormat = FileFormat {
    id: 126_822_716,
    source_type: SourceType::Wikidata,
    name: "Visual F# Source File",
    extensions: &["fs"],
    media_types: &["text/x-fsharp"],
    signatures: &[],
    related_formats: &[],
};
