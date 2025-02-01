use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866077: FileFormat = FileFormat {
    id: 27_866_077,
    puid: "wikidata/27866077",
    name: "ABC notation, version 2.1",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc", "text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
