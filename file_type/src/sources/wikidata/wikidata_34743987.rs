use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34743987: FileFormat = FileFormat {
    id: 34_743_987,
    puid: "wikidata/34743987",
    name: "Spark",
    extensions: &["spk"],
    media_types: &["archive"],
    internal_signatures: &[],
    related_formats: &[],
};
