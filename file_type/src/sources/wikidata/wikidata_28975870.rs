use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975870: FileFormat = FileFormat {
    id: 28_975_870,
    source_type: SourceType::Wikidata,
    name: "OOGL INST file",
    extensions: &["inst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
