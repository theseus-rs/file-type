use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34312718: FileFormat = FileFormat {
    id: 34_312_718,
    puid: "wikidata/34312718",
    name: "Macromedia Director, compressed Macintosh variant",
    extensions: &["dcr"],
    media_types: &["application/x-director"],
    internal_signatures: &[],
    related_formats: &[],
};
