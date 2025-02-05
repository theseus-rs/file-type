use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967088: FileFormat = FileFormat {
    id: 27_967_088,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts MUS",
    extensions: &["mus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
