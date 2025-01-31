use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110039243: FileFormat = FileFormat {
    id: 110_039_243,
    puid: "wikidata/110039243",
    name: "XIFF (Xerox Image File Format), version 2",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[],
    related_formats: &[],
};
