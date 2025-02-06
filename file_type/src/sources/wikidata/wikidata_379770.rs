use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_379770: FileFormat = FileFormat {
    id: 379_770,
    source_type: SourceType::Wikidata,
    name: "AVCHD",
    extensions: &["avchd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
