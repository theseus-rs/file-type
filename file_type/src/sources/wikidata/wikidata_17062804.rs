use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17062804: FileFormat = FileFormat {
    id: 17_062_804,
    source_type: SourceType::Wikidata,
    name: "Klip",
    extensions: &["klip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
