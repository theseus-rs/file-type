use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850401: FileFormat = FileFormat {
    id: 105_850_401,
    source_type: SourceType::Wikidata,
    name: "Poser character rigging",
    extensions: &["cr2"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
