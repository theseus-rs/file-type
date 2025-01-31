use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859090: FileFormat = FileFormat {
    id: 105_859_090,
    puid: "wikidata/105859090",
    name: "PrintFox/Pagefox bitmap (320x200)",
    extensions: &["bin", "bs"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
