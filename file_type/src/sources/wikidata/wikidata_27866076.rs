use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866076: FileFormat = FileFormat {
    id: 27_866_076,
    puid: "wikidata/27866076",
    name: "ABC notation, version 2.0",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc", "text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
