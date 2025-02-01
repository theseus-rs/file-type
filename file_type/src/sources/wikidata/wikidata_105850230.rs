use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850230: FileFormat = FileFormat {
    id: 105_850_230,
    puid: "wikidata/105850230",
    name: "EISA add-on card Configuration (with rem)",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
