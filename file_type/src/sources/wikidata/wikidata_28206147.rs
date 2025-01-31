use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206147: FileFormat = FileFormat {
    id: 28_206_147,
    puid: "wikidata/28206147",
    name: "Freedom of Press Bitamp",
    extensions: &["1", "fop"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
