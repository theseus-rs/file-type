use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1315297: FileFormat = FileFormat {
    id: 1_315_297,
    puid: "wikidata/1315297",
    name: "QuickTime VR",
    extensions: &["qtvr"],
    media_types: &["video/quicktime"],
    internal_signatures: &[],
    related_formats: &[],
};
