use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867269: FileFormat = FileFormat {
    id: 105_867_269,
    source_type: SourceType::Wikidata,
    name: "OS/2 Network Information File",
    extensions: &["nif"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
