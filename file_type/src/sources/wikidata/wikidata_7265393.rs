use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7265393: FileFormat = FileFormat {
    id: 7_265_393,
    puid: "wikidata/7265393",
    name: "QCP",
    extensions: &["qcp", "qcp"],
    media_types: &["audio/qcelp", "audio/vnd.qcelp"],
    internal_signatures: &[],
    related_formats: &[],
};
