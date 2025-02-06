use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967087: FileFormat = FileFormat {
    id: 27_967_087,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts AS4/ASF Music",
    extensions: &["as4", "asf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
