use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333907: FileFormat = FileFormat {
    id: 111_333_907,
    puid: "wikidata/111333907",
    name: "Roland MT-32 Control + PCM ROM dumps",
    extensions: &["rom"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
