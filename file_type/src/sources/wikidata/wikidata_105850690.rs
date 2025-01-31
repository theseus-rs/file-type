use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850690: FileFormat = FileFormat {
    id: 105_850_690,
    puid: "wikidata/105850690",
    name: "Kaitai Struct language (with rem)",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
