use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56827097: FileFormat = FileFormat {
    id: 56_827_097,
    puid: "wikidata/56827097",
    name: "Path of Exile GGPK",
    extensions: &["ggpk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
