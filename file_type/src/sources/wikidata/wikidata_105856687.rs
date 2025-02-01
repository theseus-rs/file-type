use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856687: FileFormat = FileFormat {
    id: 105_856_687,
    puid: "wikidata/105856687",
    name: "ObjectAid UML Explorer Class diagram",
    extensions: &["ucls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
