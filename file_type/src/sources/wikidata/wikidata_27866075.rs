use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866075: FileFormat = FileFormat {
    id: 27_866_075,
    puid: "wikidata/27866075",
    name: "ABC notation, version 1.6",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc", "text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
