use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117804274: FileFormat = FileFormat {
    id: 117_804_274,
    puid: "wikidata/117804274",
    name: "VideoImpression mini-player",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
