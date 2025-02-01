use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110238259: FileFormat = FileFormat {
    id: 110_238_259,
    puid: "wikidata/110238259",
    name: "Dramatica/StoryView Exchange",
    extensions: &["dsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
