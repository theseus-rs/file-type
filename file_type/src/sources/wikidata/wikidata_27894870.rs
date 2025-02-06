use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27894870: FileFormat = FileFormat {
    id: 27_894_870,
    source_type: SourceType::Wikidata,
    name: "Windows Media Audio Redirector",
    extensions: &["wax"],
    media_types: &["audio/x-ms-wax"],
    signatures: &[],
    related_formats: &[],
};
