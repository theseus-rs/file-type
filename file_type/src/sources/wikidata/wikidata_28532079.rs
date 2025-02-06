use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28532079: FileFormat = FileFormat {
    id: 28_532_079,
    source_type: SourceType::Wikidata,
    name: "Alchemy Format",
    extensions: &["alc"],
    media_types: &["chemical/x-alchemy"],
    signatures: &[],
    related_formats: &[],
};
