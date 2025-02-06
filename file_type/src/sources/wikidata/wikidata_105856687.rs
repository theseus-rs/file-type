use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856687: FileFormat = FileFormat {
    id: 105_856_687,
    source_type: SourceType::Wikidata,
    name: "ObjectAid UML Explorer Class diagram",
    extensions: &["ucls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
