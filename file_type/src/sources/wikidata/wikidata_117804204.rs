use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117804204: FileFormat = FileFormat {
    id: 117_804_204,
    puid: "wikidata/117804204",
    name: "VideoImpression File",
    extensions: &["vif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
