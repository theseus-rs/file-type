use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975737: FileFormat = FileFormat {
    id: 28_975_737,
    source_type: SourceType::Wikidata,
    name: "POV-Ray density file",
    extensions: &["df3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
