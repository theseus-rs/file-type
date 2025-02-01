use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_388046: FileFormat = FileFormat {
    id: 388_046,
    puid: "wikidata/388046",
    name: "X-Face",
    extensions: &["face", "xface"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
