use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5421818: FileFormat = FileFormat {
    id: 5_421_818,
    source_type: SourceType::Wikidata,
    name: "Extended Log Format",
    extensions: &["log"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
