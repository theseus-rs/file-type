use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5421818: FileFormat = FileFormat {
    id: 5_421_818,
    puid: "wikidata/5421818",
    name: "Extended Log Format",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
