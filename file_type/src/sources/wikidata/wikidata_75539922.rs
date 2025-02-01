use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75539922: FileFormat = FileFormat {
    id: 75_539_922,
    puid: "wikidata/75539922",
    name: "Ulead Private Data",
    extensions: &["upd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
